use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use crate::*;
use crate::i18n::*;
use crate::ui::components::*;

use super::{App, AppMsg};

pub fn update_main_patch(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>) {
    sender.input(AppMsg::SetDownloading(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let mut apply_patch_if_needed = true;

        let result = jadeite::get_latest()
            .and_then(|patch| patch.install(config.patch.path, clone!(@strong sender => move |state| {
                match &state {
                    InstallerUpdate::DownloadingError(err) => {
                        tracing::error!("Downloading failed: {err}");

                        sender.input(AppMsg::Toast {
                            title: tr("downloading-failed"),
                            description: Some(err.to_string())
                        });

                        // apply_patch_if_needed = false;
                    }

                    InstallerUpdate::UnpackingError(err) => {
                        tracing::error!("Unpacking failed: {err}");

                        sender.input(AppMsg::Toast {
                            title: tr("unpacking-failed"),
                            description: Some(err.clone())
                        });

                        // apply_patch_if_needed = false;
                    }
    
                    _ => ()
                }

                #[allow(unused_must_use)] {
                    progress_bar_input.send(ProgressBarMsg::UpdateFromState(state));
                }
            })));

        if let Err(err) = result {
            tracing::error!("Failed to download latest patch version");

            sender.input(AppMsg::Toast {
                title: tr("main-patch-update-failed"),
                description: Some(err.to_string())
            });

            apply_patch_if_needed = false;
        }

        sender.input(AppMsg::SetDownloading(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            apply_patch_if_needed,
            show_status_page: true
        });
    });
}
