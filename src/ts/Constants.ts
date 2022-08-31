import type { AvailableLocales } from './launcher/Locales';

import { Configs } from '../empathize';

import Game from './Game';

declare const Neutralino;
declare const NL_CWD;

class Prefix
{
    /**
     * Current prefix directory
     * 
     * @default "~/.local/share/honkers-launcher/game"
     */
    public static get current(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.prefix') as string));
    }

    /**
     * Default prefix directory
     * 
     * @default "~/.local/share/honkers-launcher/game"
     */
    public static get default(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await Paths.launcherDir}/game`));
    }

    /**
     * Change prefix directory
     * 
     * @returns promise that indicates that the prefix path has been changed in config
     */
    public static set(location: string): Promise<void>
    {
        return Configs.set('prefix', location);
    }
}

class Paths
{
    /**
     * Directory where the launcher's executable stored
     */
    public static readonly appDir: string = NL_CWD;

    /**
     * Shaders directory
     * 
     * @default "[constants.paths.appDir]/public/shaders"
     */
    public static readonly shadersDir: string = `${this.appDir}/public/shaders`;

    /**
     * Locales directory
     * 
     * @default "[constants.paths.appDir]/public/locales"
     */
    public static readonly localesDir: string = `${this.appDir}/public/locales`;

    /**
     * Launcher data directory
     * 
     * @default "~/.local/share/honkers-launcher"
     */
    public static get launcherDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await Neutralino.os.getPath('data')}/honkers-launcher`));
    }

    /**
     * Runners directory
     * 
     * @default "~/.local/share/honkers-launcher/runners"
     */
    public static get runnersDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/runners`));
    }

    /**
     * DXVKs directory
     * 
     * @default "~/.local/share/honkers-launcher/dxvks"
     */
    public static get dxvksDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/dxvks`));
    }

    /**
     * Config file
     * 
     * @default "~/.local/share/honkers-launcher/config.yaml"
     */
    public static get config(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/config.yaml`));
    }

    /**
     * Cache file
     * 
     * @default "~/.local/share/honkers-launcher/.cache.json"
     */
    public static get cache(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/.cache.json`));
    }

    public static readonly prefix = Prefix;

    /**
     * Temp directory
     * 
     * @default "~/.local/share/honkers-launcher"
     * 
     * @returns "[folders.temp] config field"
     */
    public static get tempDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.temp') as string));
    }

    /**
     * Game directory
     * 
     * @default "~/.local/share/honkers-launcher/game/drive_c/Program Files/honkers"
     * 
     * @returns "[folders.game] config field"
     */
    public static get gameDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.game') as string));
    }

    /**
     * Game data directory
     * 
     * @default "~/.local/share/honkers-launcher/game/drive_c/Program Files/honkers/[honkers]_Data"
     * 
     * @returns "[folders.game]/[honkers]_Data"
     */
    public static get gameDataDir(): Promise<string>
    {
        return new Promise(async (resolve) => {
            const folder = await Game.server === 'global' ?
                constants.placeholders.uppercase.first :
                constants.placeholders.uppercase.full.cn;

            resolve(`${await this.gameDir}/${folder}_Data`);
        });
    }
}

export default class constants
{
    public static readonly placeholders = {
        uppercase:
        {
            /**
             * Anime
             */
            first: atob('Qkgz'),

            /**
             * Game
             */
            second: atob('SW1wYWN0'),

            /**
             * Anime Game
             */
            full: {
                global: atob('SG9ua2FpIEltcGFjdCAzcmQ='),
                cn: atob('WXVhblNoZW4=')
            },

            /**
             * anAnimeCompany
             */
            company: atob('bWlIb1lv'),

            /**
             * NOTAREALANIMECOMPANY
             */
            company_alterego: atob('Q09HTk9TUEhFUkU=')
        },

        lowercase:
        {
            /**
             * anime
             */
            first: atob('aG9ua2FpaW1wYWN0Mw=='),

            /**
             * animecompany
             */
            company: atob('bWlob3lv'),

            /**
             * newanimecompany
             */
            company_new: atob('aG95b3ZlcnNl')
        }
    };

    public static readonly api = {
        key: {
            global: 'gcStgarh',
            cn: 'eYd89JmJ'
        },
        launcher_id: {
            global: 10,
            cn: 18
        }
    };

    public static readonly uri = {
        api: {
            global: `https://sdk-os-static.${this.placeholders.lowercase.company_new}.com/bh3_global/mdk/launcher/api`,
            cn: `https://sdk-static.${this.placeholders.lowercase.company}.com/hk4e_cn/mdk/launcher/api`
        },
        telemetry: {
            global: [
                atob('bG9nLXVwbG9hZC1vcy5ob3lvdmVyc2UuY29t'),
                atob('b3ZlcnNlYXVzcGlkZXIueXVhbnNoZW4uY29t')
            ],
            cn: [
                atob('bG9nLXVwbG9hZC5taWhveW8uY29t'),
                atob('dXNwaWRlci55dWFuc2hlbi5jb20=')
            ]
        },
        winetricks: 'https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks',
        launcher: 'https://github.com/an-anime-team/honkers-launcher',
        discord: 'https://discord.gg/ck37X6UWBp',
        dxvk_list: 'https://github.com/an-anime-team/honkers-launcher/raw/main/public/dxvks.yaml',
        runners_list: 'https://github.com/an-anime-team/honkers-launcher/raw/main/public/runners.yaml'
    };

    public static readonly paths = Paths;

    public static versionsUri(server: 'global' | 'cn'): string
    {
        return `${this.uri.api[server]}/resource?key=${this.api.key[server]}&launcher_id=${this.api.launcher_id[server]}`;
    }

    public static backgroundUri(server: 'global' | 'cn', lang: AvailableLocales): string
    {
        return `${this.uri.api[server]}/content?filter_adv=true&key=${this.api.key[server]}&launcher_id=${this.api.launcher_id[server]}&language=${lang}`;
    }
}
