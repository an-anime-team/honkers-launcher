// SPDX-License-Identifier: GPL-3.0-or-later
//
// honkers-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::process::Command;

use anime_launcher_sdk::anime_game_core::honkai::prelude::*;
use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::anime_game_core::minreq;

use agl_locale::unic_langid::LanguageIdentifier;
use md5::{Digest, Md5};

use crate::STARTUP_LANG;
use crate::i18n::format_lang;

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

fn get_expected_edition(lang: &LanguageIdentifier) -> GameEdition {
    match lang.language.as_str() {
        "ja" => GameEdition::Japan,
        "ko" => GameEdition::Korea,

        "zh" => match lang.region {
            Some(region) => {
                if region.as_str().eq_ignore_ascii_case("tw") {
                    GameEdition::Taiwan
                }
                else {
                    GameEdition::China
                }
            }

            None => GameEdition::China
        },

        "vi" | "th" | "id" => GameEdition::Sea,

        _ => GameEdition::Global
    }
}

#[cached::proc_macro::cached]
pub fn get_background_info() -> anyhow::Result<Background> {
    let lang = &STARTUP_LANG;

    let expected_edition = get_expected_edition(lang);

    let uri = match expected_edition {
        GameEdition::China => concat!(
            "https://hyp-api.",
            "mi",
            "ho",
            "yo",
            ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=jGHBHlcOq1"
        )
        .to_string(),

        _ => {
            let uri = concat!(
                "https://sg-hyp-api.",
                "ho",
                "yo",
                "verse",
                ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=VYTpXlbWo8&language="
            );

            format!("{uri}{}", format_lang(lang))
        }
    };

    let json = serde_json::from_slice::<serde_json::Value>(minreq::get(uri).send()?.as_bytes())?;

    let uri = json["data"]["game_info_list"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to list games in the backgrounds API"))?
        .iter()
        .find(|game| game["game"]["id"].as_str() == Some(expected_edition.api_game_id()))
        .ok_or_else(|| anyhow::anyhow!("Failed to find the game in the backgrounds API"))?["backgrounds"]
        .as_array()
        .and_then(|backgrounds| backgrounds.iter().next())
        .and_then(|background| background["background"]["url"].as_str())
        .map(|background| background.to_owned());

    let Some(uri) = uri
    else {
        anyhow::bail!("Failed to get background picture url");
    };

    let hash = uri
        .split('/')
        .next_back()
        .unwrap_or_default()
        .split('_')
        .next()
        .unwrap_or_default()
        .to_owned();

    Ok(Background {
        uri,
        hash
    })
}

pub fn download_background() -> anyhow::Result<()> {
    tracing::debug!("Downloading background picture");

    let info = get_background_info()?;

    let mut download_image = true;

    if crate::BACKGROUND_FILE.exists() {
        let mut hash = [0; 16];

        hash.copy_from_slice(&Md5::digest(std::fs::read(crate::BACKGROUND_FILE.as_path())?));

        let hash = u128::from_le_bytes(hash);

        if format!("{hash:x}").eq_ignore_ascii_case(&info.hash) {
            tracing::debug!("Background picture is already downloaded. Skipping");

            download_image = false;
        }
    }

    if download_image {
        let mut downloader = Downloader::new(&info.uri)?;

        downloader.continue_downloading = false;

        if let Err(err) = downloader.download(crate::BACKGROUND_FILE.as_path(), |_, _| {}) {
            anyhow::bail!(err);
        }
    }

    // Workaround for GTK weakness
    if info.uri.ends_with(".webp") {
        Command::new("dwebp")
            .arg(crate::BACKGROUND_FILE.as_path())
            .arg("-o")
            .arg(crate::PROCESSED_BACKGROUND_FILE.as_path())
            .spawn()?
            .wait()?;

        // If it failed to re-code the file - just copy it
        // Will happen with HSR because devs apparently named
        // their background image ".webp" while it's JPEG
        if !crate::PROCESSED_BACKGROUND_FILE.exists() {
            std::fs::copy(
                crate::BACKGROUND_FILE.as_path(),
                crate::PROCESSED_BACKGROUND_FILE.as_path()
            )?;
        }
    }
    else {
        std::fs::copy(
            crate::BACKGROUND_FILE.as_path(),
            crate::PROCESSED_BACKGROUND_FILE.as_path()
        )?;
    }

    Ok(())
}
