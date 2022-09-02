import type Launcher from '../../Launcher';
import type { Stream as DownloadingStream } from '@empathize/framework/dist/network/Downloader';

declare const Neutralino;

import AbstractInstaller from '../../core/AbstractInstaller';
import constants from '../../Constants';
import Runners from '../../core/Runners';
import { Process, promisify, fs  } from '../../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

class Stream extends AbstractInstaller
{
    public constructor(uri: string, path: string)
    {
        super(uri, path);
    }
}

function installmf(uri: string, path: string): Promise<Stream|null> {
    return new Promise((resolve, reject) => {
        resolve(new Stream(uri, path));
    });
}

function apply(prefix: string): Promise<void> {
    return new Promise(async (resolve) => {
        const debugThread = new DebugThread('MFPlat.apply', `Applying MFPlat Patch`);
            
        const mfPlatDir = `${await constants.paths.launcherDir}/mfplat`;
        const runner = await Runners.current();
        const runnerDir = `${await constants.paths.runnersDir}/${runner?.name}`;
        const gameDir = await constants.paths.gameDir;
        if (!await fs.exists(gameDir))
            await fs.mkdir(gameDir);

        const pipeline = promisify({
            callbacks: [
                () => Neutralino.os.execCommand(`chmod +x "${mfPlatDir}/install-mf.sh"`),
                (): Promise<void> => new Promise(async (resolve) => {
                    const alias = runner ? `alias winecfg="${runnerDir}/${runner.files.winecfg}"` : 'true';

                    Process.run(`sh -c '${alias};./install-mf.sh'`, {
                        cwd: mfPlatDir,
                        env: {
                            WINE: runner ? `${runnerDir}/${runner.files.wine}` : 'wine',
                            WINE64: runner ? `${runnerDir}/${runner.files.wine64}` : 'wine64',
                            WINESERVER: runner ? `${runnerDir}/${runner.files.wineserver}` : 'wineserver',
                            WINEPREFIX: prefix
                        }
                    }).then((process) => {
                        let processOutput = '';

                        process.output((output) => processOutput += output);

                        process.finish(() => {
                            debugThread.log({
                                message: [
                                    'Patch script output:',
                                    processOutput.split(/\r\n|\r|\n/)
                                ]
                            });

                            resolve();
                        });
                    });
                })
            ]
        });

        pipeline.then(() => {
            fs.copy(`${mfPlatDir}/mfplat.dll`, `${gameDir}/mfplat.dll`);
            fs.remove(`${mfPlatDir}/windows6.1-kb976932-x64_74865ef2562006e51d7f9333b4a8d45b7a749dab.exe`);

            debugThread.log('Applying completed');

            resolve();
        });
    });
}

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        // Create prefix if it is not created
        import('./CreatePrefix').then(async (module) => {
            module.default(launcher).then(async () => {
                const mfPlatDir = `${await constants.paths.launcherDir}/mfplat`;
                await Neutralino.filesystem.createDirectory(mfPlatDir);

                installmf(`https://github.com/an-anime-team/honkers-launcher/raw/main/repository/mfplat.zip`, mfPlatDir).then((stream) => {
                    launcher.progressBar?.init({
                        label: 'Downloading MFPlat Patch...',
                        showSpeed: true,
                        showEta: true,
                        showPercents: true,
                        showTotals: true
                    });

                    stream?.downloadStart(() => launcher.progressBar?.show());

                    stream?.downloadProgress((current: number, total: number, difference: number) => {
                        launcher.progressBar?.update(current, total, difference);
                    });

                    let unpacking = true;

                    stream?.unpackStart(() => {
                        launcher.progressBar?.init({
                            label: () => unpacking ? 'Unpacking and Applying MFPlat Patch...' : 'Applying MFPlat Patch...',
                            showSpeed: true,
                            showEta: true,
                            showPercents: true,
                            showTotals: true
                        });
                    });

                    stream?.unpackProgress((current: number, total: number, difference: number) => {
                        launcher.progressBar?.update(current, total, difference);
                    });

                    stream?.unpackFinish(async () => {
                        unpacking = false;
                        apply(await constants.paths.prefix.current).then(() => {
                            launcher.progressBar?.hide();

                            resolve();
                        });
                    });
                });
            });
        });
    });
};
