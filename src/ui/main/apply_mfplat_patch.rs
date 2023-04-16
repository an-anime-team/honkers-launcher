use relm4::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::honkai::config::Config;

use anime_launcher_sdk::anime_game_core::honkai::linux_patch::MfplatPatch;

use crate::i18n::*;
use super::{App, AppMsg};

pub fn apply_mfplat_patch(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let mut apply_patch_if_needed = true;

        if let Err(err) = MfplatPatch::apply(config.game.wine.prefix) {
            tracing::error!("Failed to patch the game");

            sender.input(AppMsg::Toast {
                title: tr("game-patching-error"),
                description: Some(err.to_string())
            });

            // Don't try to apply the patch after state updating
            // because we just failed to do it
            apply_patch_if_needed = false;
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            apply_patch_if_needed,
            show_status_page: true
        });
    });
}
