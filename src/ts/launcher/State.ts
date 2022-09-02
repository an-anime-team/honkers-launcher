import { get as svelteget } from 'svelte/store';
import { dictionary, locale } from 'svelte-i18n';

import semver from 'semver';

import { Windows, Debug, IPC, Notification, fs, path } from '../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import type { LauncherState } from '../types/Launcher';

import Launcher from '../Launcher';
import Game from '../Game';
import Runners from '../core/Runners';
import DXVK from '../core/DXVK';
import Locales from './Locales';
import Git from '../core/Git';
import constants from '../Constants';
import Background from './Background';

declare const Neutralino;

export default class State
{
    public launcher: Launcher;

    public backgroundImage: HTMLElement;
    public socialsIframe: HTMLElement;

    public launchButton: HTMLElement;
    public pauseButton: HTMLElement;
    public predownloadButton: HTMLElement;
    public settingsButton: HTMLElement;

    protected _state: LauncherState = 'game-launch-available';

    protected events = {
        'runner-installation-required': import('./states/InstallWine'),
        'dxvk-installation-required': import('./states/InstallDXVK'),

        'game-installation-available': import('./states/Install'),
        'game-update-available': import('./states/Install'),

        'game-pre-installation-available': import('./states/Launch'),
        'game-launch-available': import('./states/Launch')
    };

    public constructor(launcher: Launcher)
    {
        this.launcher = launcher;

        this.backgroundImage = <HTMLElement> document.getElementById('background');
        this.socialsIframe   = <HTMLElement> document.getElementById('social-iframe');

        this.launchButton      = <HTMLElement> document.getElementById('launch');
        this.pauseButton       = <HTMLElement> document.getElementById('pause');
        this.predownloadButton = <HTMLElement> document.getElementById('predownload');
        this.settingsButton    = <HTMLElement> document.getElementById('settings');

        Background.get().then((uri) => {
            if (uri)
                this.backgroundImage.setAttribute('src', uri);
        });

        launcher.getSocial().then((uri) => this.socialsIframe.setAttribute('src', uri));

        this.launchButton.onclick = () => {
            if (this.events[this._state])
            {
                this.launchButton.style['display'] = 'none';
                this.settingsButton.style['display'] = 'none';

                this.events[this._state].then((event) => {
                    event.default(this.launcher).then(() => {
                        this.update().then(() => {
                            this.launchButton.style['display'] = 'block';
                            this.settingsButton.style['display'] = 'block';
                        });
                    });
                });
            }
        };

        this.predownloadButton.onclick = () => {
            this.launchButton.style['display'] = 'none';
            this.predownloadButton.style['display'] = 'none';
            this.settingsButton.style['display'] = 'none';

            // We must specify this files here directly
            // because otherwise Vite will not bundle 'em
            const predownloadModule = import('./states/Predownload');

            (this._state === 'game-pre-installation-available' ? predownloadModule : '')
                .then((module) => {
                    module.default(this.launcher).then(() => {
                        this.update().then(() => {
                            this.launchButton.style['display'] = 'block';
                            this.settingsButton.style['display'] = 'block';
                        });
                    });
                });
        };

        this.update().then(async () => {
            // Close splash screen
            IPC.write('launcher-loaded');

            // Show launcher's window
            await Windows.current.show();
            await Windows.current.center(1280, 700);

            // Check for new versions of the launcher
            Git.getTags(constants.uri.launcher).then((tags) => {
                for (const tag of tags.reverse())
                    if (semver.gt(tag.tag, Launcher.version))
                    {
                        const locales = Locales.translate<object>('notifications.launcher_update_available');
                        
                        Notification.show({
                            title: locales['title'].replace('{from}', Launcher.version).replace('{to}', tag.tag),
                            body:  locales['body'].replace('{repository}', constants.uri.launcher),
                            icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`
                        });

                        break;
                    }
            });

            // This thing will fix window resizing
            // in several cases (wayland + gnome + custom theme)
            const resizer = () => {
                if (window.innerWidth < 1000)
                    setTimeout(resizer, 10);

                else
                {
                    Windows.current.setSize({
                        width: 1280 + (1280 - window.innerWidth),
                        height: 700 + (700 - window.innerHeight),
                        resizable: false
                    });
                }
            }

            setTimeout(resizer, 10);
        });

        Locales.bind((message) => this.updateLocales(message as object), 'launcher.states');
    }

    /**
     * Get current launcher state
     */
    public get(): LauncherState
    {
        return this._state;
    }

    /**
     * Set launcher state
     */
    public set(state: LauncherState): void
    {
        this._state = state;

        this.launcher.progressBar!.hide();
        this.predownloadButton.style['display'] = 'none';

        this.launchButton.classList.remove('button-blue');
        this.launchButton.setAttribute('aria-label', '');

        const currentDictionary = svelteget(dictionary);
        const currentLocale = svelteget(locale);

        this.updateLocales((currentDictionary[currentLocale ?? 'en-us'] ?? currentDictionary['en-us'])['launcher']!['states'] as object);
    }

    /**
     * Update components texts
     */
    public updateLocales(dictionary: object)
    {
        /*Debug.log({
            function: 'State.updateLocales',
            message: `Updated locales: ${JSON.stringify(dictionary)}`
        });*/

        switch(this._state)
        {
            case 'runner-installation-required':
                this.launchButton.textContent = dictionary['installation']['install_wine'];

                break;

            case 'dxvk-installation-required':
                this.launchButton.textContent = dictionary['installation']['install_dxvk'];

                break;
            
            case 'game-launch-available':
                this.launchButton.textContent = dictionary['ready']['launch'];

                break;

            case 'game-pre-installation-available':
                this.predownloadButton.style['display'] = 'block';

                this.launchButton.textContent = dictionary['ready']['launch'];

                break;

            case 'game-installation-available':
                this.launchButton.textContent = dictionary['installation']['install'];

                break;

            case 'game-update-available':
                this.launchButton.textContent = dictionary['installation']['update'];

                break;
        }
    }

    /**
     * Update launcher state
     * 
     * @returns new launcher state
     * 
     * This state will be automatically applied to the launcher
     * so you don't need to do it manually
     */
    public update(): Promise<string>
    {
        const debugThread = new DebugThread('State.update', 'Updating launcher state');

        return new Promise(async (resolve) => {
            let state: LauncherState|null = null;

            const runner = await Runners.current();

            // Check if the wine is installed
            if (runner === null)
            {
                debugThread.log('Runner is not specified');

                state = 'runner-installation-required';

                Runners.list().then((list) => {
                    for (const family of list)
                        for (const runner of family.runners)
                            if (runner.installed && runner.recommended)
                            {
                                debugThread.log(`Automatically selected runner ${runner.title} (${runner.name})`);

                                state = null;

                                Runners.current(runner).then(() => {
                                    this.update().then(resolve);
                                });

                                return;
                            }
                });

                if (state !== null)
                {
                    debugThread.log('No recommended runner installed');

                    this.set(state);

                    resolve(state);
                }
            }

            else
            {
                const dxvk = await DXVK.current();

                // Check if the DXVK is installed
                if (dxvk === null)
                {
                    debugThread.log('DXVK is not specified');

                    state = 'dxvk-installation-required';

                    DXVK.list().then((list) => {
                        for (const dxvk of list)
                            if (dxvk.installed && dxvk.recommended)
                            {
                                debugThread.log(`Automatically selected DXVK ${dxvk.version}`);

                                state = null;

                                DXVK.current(dxvk).then(() => {
                                    this.update().then(resolve);
                                });

                                return;
                            }
                    });

                    if (state !== null)
                    {
                        debugThread.log('No recommended DXVK installed');

                        this.set(state);

                        resolve(state);
                    }
                }

                // Otherwise select some launcher state
                else
                {
                    const gameCurrent = await Game.current;
                    
                    if (gameCurrent === null)
                        state = 'game-installation-available';

                    else
                    {
                        const gameLatest = await Game.getLatestData();

                        if (gameCurrent != gameLatest.game.latest.version)
                            state = 'game-update-available';
                        
                        else
                        {
                            state = 'game-launch-available';
                        }
                    }

                    debugThread.log(`Updated state: ${state}`);

                    this.set(state);

                    resolve(state);
                }
            }
        });
    }
};
