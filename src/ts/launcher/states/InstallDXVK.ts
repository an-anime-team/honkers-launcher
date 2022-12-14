import type Launcher from '../../Launcher';

import DXVK from '../../core/DXVK';
import constants from '../../Constants';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        // Create prefix if it is not created
        import('./CreatePrefix').then((module) => {
            module.default(launcher).then(() => {
                // And then download the DXVK
                DXVK.download('honk-1.9.4').then((stream) => {
                    launcher.progressBar?.init({
                        label: 'Downloading DXVK honk-1.9.4...',
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
                            label: () => unpacking ? 'Unpacking DXVK honk-1.9.4...' : 'Applying DXVK honk-1.9.4...',
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

                        // Select this DXVK
                        await DXVK.current('honk-1.9.4');

                        // And apply it
                        DXVK.apply(await constants.paths.prefix.current, 'honk-1.9.4').then(() => {
                            import('./CreatePrefix').then((module) => {
                                module.default(launcher).then(() => {
                                    import('./InstallMF').then((module) => {
                                        module.default(launcher).then(() => resolve());
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });
    });
};
