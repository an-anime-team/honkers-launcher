import type Launcher from '../../Launcher';

import Game from '../../Game';
import Prefix from '../../core/Prefix';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Prefix.exists().then((exists) => {
            if (!exists)
            {
                import('./CreatePrefix').then((module) => {
                    module.default(launcher).then(() => updateGame());
                });
            }

            else updateGame();
        });

        const updateGame = async () => {
            const prevGameVersion = await Game.current;

            Game.predownloadUpdate(prevGameVersion).then((stream) => {
                launcher.progressBar?.init({
                    label: 'Pre-downloading game...',
                    showSpeed: true,
                    showEta: true,
                    showPercents: true,
                    showTotals: true
                });

                stream?.start(() => launcher.progressBar?.show());
    
                stream?.progress((current: number, total: number, difference: number) => {
                    launcher.progressBar?.update(current, total, difference);
                });
    
                stream?.finish(() => {
                    resolve();
                });
            });
        };
    });
};
