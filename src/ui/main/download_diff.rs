use anime_launcher_sdk::anime_game_core::honkai::version_diff::*;
use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::honkai::config::Config;
use gtk::glib::clone;
use relm4::Sender;
use relm4::prelude::*;
use anime_launcher_sdk::anime_game_core::sophon::installer::Update as SophonInstallerUpdate;
use anime_launcher_sdk::anime_game_core::sophon::updater::Update as SophonPatcherUpdate;

use super::{App, AppMsg};
use crate::ui::components::*;
use crate::*;

pub fn download_diff(
    sender: ComponentSender<App>,
    progress_bar_input: Sender<ProgressBarMsg>,
    mut diff: VersionDiff
) {
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let config = Config::get().unwrap();
        let game_path = config
            .game
            .path
            .for_edition(config.launcher.edition)
            .to_path_buf();

        if !game_path.exists() {
            if let Err(err) = std::fs::create_dir(&game_path) {
                tracing::error!(?err, "Failed to create game directory");
                sender.input(AppMsg::Toast {
                    title: tr!("downloading-failed"),
                    description: Some(err.to_string())
                });
            }
        }

        if let Some(temp) = config.launcher.temp {
            diff = diff.with_temp_folder(temp);
        }

        let result = diff.install_to(
            game_path,
            config.launcher.sophon.threads as usize,
            clone!(
                #[strong]
                sender,
                move |state| {
                    match &state {
                        DiffUpdate::Installer(SophonInstallerUpdate::DownloadingError(err))
                        | DiffUpdate::Patcher(SophonPatcherUpdate::DownloadingError(err)) => {
                            tracing::error!("Downloading failed: {err}");

                            sender.input(AppMsg::Toast {
                                title: tr!("downloading-failed"),
                                description: Some(err.to_string())
                            });
                        }

                        _ => ()
                    }

                    #[allow(unused_must_use)]
                    {
                        progress_bar_input.send(ProgressBarMsg::UpdateFromDiffState(state));
                    }
                }
            )
        );

        let mut perform_on_download_needed = true;

        if let Err(err) = result {
            tracing::error!("Downloading failed: {err}");

            sender.input(AppMsg::Toast {
                title: tr!("downloading-failed"),
                description: Some(err.to_string())
            });

            // Don't try to download something after state updating
            // because we just failed to do it
            perform_on_download_needed = false;
        }

        sender.input(AppMsg::SetDownloading(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed,
            show_status_page: false
        });
    });
}
