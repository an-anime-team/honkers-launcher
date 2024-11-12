use std::process::Command;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::anime_game_core::minreq;

use md5::{Md5, Digest};

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

pub fn get_uri() -> String {
    let lang = crate::i18n::get_lang();

    if lang.language == unic_langid::langid!("zh-cn").language {
        concat!("https://hyp-api.", "mi", "ho", "yo", ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=jGHBHlcOq1").to_owned()
    }

    else {
        let uri = concat!("https://sg-hyp-api.", "ho", "yo", "verse", ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=VYTpXlbWo8&language=");

        uri.to_owned() + &crate::i18n::format_lang(lang)
    }
}

#[cached::proc_macro::cached(result)]
pub fn get_background_info() -> anyhow::Result<Background> {
    let lang = crate::i18n::get_lang();
    let gameid: &str;
    
    if lang.language == unic_langid::langid!("en-us").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("ru-ru").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("de-de").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("fr-fr").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("es-es").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("tr-tr").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("it-it").language
    {
        gameid = "5TIVvvcwtM"
    }
    else if lang.language == unic_langid::langid!("zh-cn").language
    {
        gameid = "osvnlOc0S8"
    }
    else if lang.language == unic_langid::langid!("ja-jp").language
    {
        gameid = "g0mMIvshDb"
    }
    else if lang.language == unic_langid::langid!("ko-kr").language
    {
        gameid = "uxB4MC7nzC"
    }
    else if lang.language == unic_langid::langid!("id-id").language
    {
        gameid = "bxPTXSET5t"
    }
    else if lang.language == unic_langid::langid!("vi-vn").language
    {
        gameid = "bxPTXSET5t"
    }
    else if lang.language == unic_langid::langid!("th-th").language
    {        
        gameid = "bxPTXSET5t"
    }
    else
    {
        gameid = "5TIVvvcwtM"
    }


    let json = serde_json::from_slice::<serde_json::Value>(minreq::get(get_uri()).send()?.as_bytes())?;

    let uri = json["data"]["game_info_list"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to list games in the backgrounds API"))?
        .iter()
        .find(|game| {
            match game["game"]["id"].as_str() {
                Some(id) => id.starts_with(gameid),
                _ => false
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Failed to find the game in the backgrounds API"))?["backgrounds"]
        .as_array()
        .and_then(|backgrounds| backgrounds.iter().next())
        .and_then(|background| background["background"]["url"].as_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get background picture url"))?
        .to_string();

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
