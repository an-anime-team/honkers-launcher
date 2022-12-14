import { fetch } from '../../empathize';

import constants from '../Constants';
import Locales from './Locales';
import Game from '../Game';

export default class Background
{
    /**
     * Get background uri
     * 
     * @return null if uri is not available
     */
    public static get(): Promise<string|null>
    {
        return new Promise(async (resolve) => {
            const server = await Game.server;
            const locale = Locales.fallback((await Locales.default()) ?? 'en-us');

            const tryToFetch = (server: 'global' | 'cn'): Promise<string|null> => {
                return new Promise((resolve) => {
                    fetch(constants.backgroundUri(server, locale))
                        .then((header) => header.body().then((body) => {
                            const data = JSON.parse(body);

                            if (data.message !== 'OK' || data.data.adv === null)
                            {
                                if (server === 'global')
                                    resolve(null);

                                else tryToFetch('global').then(resolve);
                            }

                            else resolve(data.data.adv.background);
                        }));
                });
            };

            tryToFetch(server).then(resolve);
        });
    }

    /**
     * Get background image URI
     * 
     * Neutralino is unnable to load local files
     * so we sadly can't provide proper caching
     */
    /*public getBackgroundUri(): Promise<string>
    {
        return new Promise((resolve, reject) => {
            Cache.get('background').then(async (background) => {
                const launcherDir = await constants.paths.launcherDir;

                // If the background is not cached or
                // the cache is expired
                if (background === null || background.expired)
                {
                    const header = await fetch(constants.backgroundUri + await Configs.get('lang.launcher'));

                    // Reject an error if background server is not available
                    if (!header.ok)
                        reject(new Error(`${constants.placeholders.uppercase.company}'s background server is not responding`));

                    else
                    {
                        header.body().then(async (body) => {
                            const json = JSON.parse(body);

                            // If the background wasn't loaded - then again reject an error
                            if (json.data.adv.background === undefined)
                                reject(new Error('Background property wasn\'t found'));
                            
                            else
                            {
                                // Store some background info to the cache
                                await Cache.set('background', {
                                    gameVersion: (await Game.latest).version,
                                    cachedAt: Math.round(Date.now() / 1000)
                                }, 7 * 24 * 60 * 60);

                                console.log(json.data.adv.background);

                                // Download background picture and return path to it
                                Downloader.download(json.data.adv.background, `${launcherDir}/background.png`)
                                    .then((stream) => {
                                        stream.finish(() => resolve(`file://${launcherDir}/background.png`));
                                    });
                            }
                        });
                    }
                }

                // Background is cached
                // todo: add cache auto dropping when the banner is updated
                else resolve(`file://${launcherDir}/background.png`);
            });
        });
    }*/
};
