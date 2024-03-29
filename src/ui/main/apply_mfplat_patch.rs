use relm4::prelude::*;

use anime_launcher_sdk::anime_game_core::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::honkai::config::Config;

use crate::*;

use super::{App, AppMsg};

pub fn apply_mfplat_patch(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let temp = config.launcher.temp.clone()
            .unwrap_or_else(std::env::temp_dir);

        if let Err(err) = mfplat::apply(config.get_wine_prefix_path(), temp) {
            tracing::error!("Failed to patch the game");

            sender.input(AppMsg::Toast {
                title: tr!("game-patching-error"),
                description: Some(err.to_string())
            });
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true
        });
    });
}
