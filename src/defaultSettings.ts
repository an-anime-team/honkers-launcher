import { Configs } from './empathize';

import constants from './ts/Constants';
import Locales from './ts/launcher/Locales';

export default new Promise<void>(async (resolve) => {
    const systemLocale = await Locales.system();
    
    await Configs.defaults({
        lang: {
            launcher: systemLocale,
        },

        /**
         * Game server
         * 
         * Available options: "global" and "cn"
         */
        server: 'global',

        folders: {
            /**
             * Path to wine prefix
             * 
             * @default "~/.local/share/honkers-launcher/game"
             */
            prefix: await constants.paths.prefix.default,

            /**
             * Path to game installation folder
             * 
             * @default "~/.local/share/honkers-launcher/game/drive_c/Program Files/[Honkers Game]"
             */
            game: `${await constants.paths.prefix.default}/drive_c/Program Files/${constants.placeholders.uppercase.first}`,
            
            /**
             * Path to some temp folder
             * 
             * @default "~/.local/share/honkers-launcher"
             */
            temp: await constants.paths.launcherDir
        },
    
        /**
         * Runner name to use, or null if runner is not specified
         * 
         * @default null
         */
        runner: null,
    
        /**
         * DXVK name to use, or null if DXVK is not specified
         * 
         * @default null
         */
        dxvk: null,

        /**
         * Environment variables
         * 
         * null to disable any
         * 
         * @default null
         */
        env: null,
    
        /**
         * Launcher theme
         * 
         * Can be "system", "light" and "dark"
         * 
         * @default "system"
         */
        theme: 'system',
    
        /**
         * HUD
         * 
         * "none" if not in use. Otherwise it's "dxvk" or "mangohud"
         * 
         * @default "none"
         */
        hud: 'none',

        /**
         * Wine-related configs
         */
        wine: {
            /**
             * Wine synchronization
             * 
             * Available options: none, esync, fsync
             * 
             * @default "esync"
             */
            sync: 'esync',

            /**
             * If the launcher should enable AMD FSR
             * 
             * @default true
             */
            fsr: true,

            /**
             * Wine Virtual Desktop
             */
            virtual_desktop: {
                /**
                 * If it is enabled
                 * 
                 * @default false
                 */
                enabled: false,

                /**
                 * Virtual Desktop Width
                 * 
                 * @default 720
                 */
                width: 1280,

                /**
                 * Virtual Desktop Height
                 * 
                 * @default 1280
                 */
                height: 720
            }
        },
    
        /**
         * vkBasalt preset to use
         * 
         * "none" if not in use. Otherwise it should be a folder name from the "shaders" folder
         * 
         * @default "none"
         */
        shaders: 'none',

        /**
         * Discord RPC integration
         */
        discord: {
            /**
             * If it is enabled
             * 
             * @default false
             */
            enabled: false,

            /**
             * Should it display amount of spent time or not
             * 
             * @default true
             */
            timer: true,

            /**
             * Discord RPC states
             */
            states: {
                'in-launcher': {
                    details: 'Preparing to launch',
                    state: '',
                    icon: 'launcher'
                },

                'in-game': {
                    details: 'Trying to Survive',
                    state: 'the Honkai',
                    icon: 'main'
                }
            }
        },

        /**
         * If the launcher should use GameMode
         * 
         * @default false
         */
        gamemode: false,

        /**
         * Allow Borderless Window
         * 
         * Whether the game launches with the parameters "-screen-fullscreen 0 -popupwindow"
         * This allows the game to be played in a borderless window by selecting fullscreen and pressing alt+enter
         * 
         * @default false
         */
        borderless_window: false,

        /**
         * Use separate terminal window to run the wine command
         * 
         * It'll try to use some of the supported default terminal applications. Otherwise the option will not have an effect
         * 
         * More details in the `Launch.ts` script
         * 
         * @default false
         */
        use_terminal: false,

        /**
         * If the launcher should automatically delete log files
         */
        purge_logs: {
            /**
             * Should launcher delete some game logs (DXVK .log and .dmp files)
             * 
             * @default true
             */
            game: true,

            /**
             * Period of time launcher should delete its logs
             * 
             * Can be in "*d" format, where * is amount of days (e.g. 5d = 5 days), or "never" to never delete logs
             * 
             * @default "5d"
             */
            launcher: '5d'
        }
    });

    await Configs.flush();

    resolve();
});
