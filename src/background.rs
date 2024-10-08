use std::process::Command;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::anime_game_core::minreq;

use md5::{Md5, Digest};

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

#[cached::proc_macro::cached(result)]
pub fn get_background_info() -> anyhow::Result<Background> {
    let lang = crate::i18n::get_lang();

    let uri = if lang.language == unic_langid::langid!("zh-cn").language {
        let api_uri = concat!("https://hyp-api.", "mi", "ho", "yo", ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=jGHBHlcOq1");

        let json = serde_json::from_slice::<serde_json::Value>(minreq::get(api_uri).send()?.as_bytes())?;

        json["data"]["game_info_list"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Failed to list games in the backgrounds API"))?
            .iter()
            .find(|game| {
                match game["game"]["biz"].as_str() {
                    Some(biz) => biz.starts_with("bh3_"),
                    _ => false
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find the game in the backgrounds API"))?["backgrounds"]
            .as_array()
            .and_then(|backgrounds| backgrounds.iter().next())
            .and_then(|background| background["background"]["url"].as_str())
            .map(|background| background.to_owned())
    } else {
        let api_uri = concat!("https://bh3-launcher.", "ho", "yo", "verse", ".com/bh3_global/mdk/launcher/api/content?filter_adv=true&key=dpz65xJ3&launcher_id=10&language=");

        let json = serde_json::from_slice::<serde_json::Value>(minreq::get(api_uri.to_string() + &crate::i18n::format_lang(lang)).send()?.as_bytes())?;

        json["data"]["adv"]["background"].as_str().map(|background| background.to_owned())
    };
    
    let Some(uri) = uri else {
        anyhow::bail!("Failed to get background picture url");
    };

    let hash = uri.split('/')
        .last()
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
        let hash = Md5::digest(std::fs::read(crate::BACKGROUND_FILE.as_path())?);

        if format!("{:x}", hash).to_lowercase() == info.hash {
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
            std::fs::copy(crate::BACKGROUND_FILE.as_path(), crate::PROCESSED_BACKGROUND_FILE.as_path())?;
        }
    }

    else {
        std::fs::copy(crate::BACKGROUND_FILE.as_path(), crate::PROCESSED_BACKGROUND_FILE.as_path())?;
    }

    Ok(())
}
