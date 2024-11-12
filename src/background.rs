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
    if lang.language == unic_langid::langid!("en-us").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "fbwQurHDcs"
    }
    else if lang.language == unic_langid::langid!("ru-ru").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "leRCQKF15s"
    }
    else if lang.language == unic_langid::langid!("de-de").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "OhoenPvEh6"
    }
    else if lang.language == unic_langid::langid!("fr-fr").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "Unc0cPndqv"
    }
    else if lang.language == unic_langid::langid!("es-es").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "toeETy4CzB"
    }
    else if lang.language == unic_langid::langid!("tr-tr").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "vt5uLr7FZK"
    }
    else if lang.language == unic_langid::langid!("it-it").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "JnypvGMwdi"
    }
    else if lang.language == unic_langid::langid!("id-id").language
    {
        let gameid = "5TIVvvcwtM"
        let bgid = "4FSfWnEy8k"
    }
    else if lang.language == unic_langid::langid!("ja-jp").language
    {
        let gameid = "g0mMIvshDb"
        let bgid = "8UVTQB5f37"
    }
    else if lang.language == unic_langid::langid!("ko-kr").language
    {
        let gameid = "uxB4MC7nzC"
        let bgid = "uyBfZfkFo2"
    }
    else if lang.language == unic_langid::langid!("vi-vn").language
    {
        let gameid = "bxPTXSET5t"
        let bgid = "V7MQ05c4SO"
    }
    else if lang.language == unic_langid::langid!("th-th").language
    {        
        let gameid = "bxPTXSET5t"
        let bgid = "M5XMQXkex7"
    }
    else
    {
        let gameid = "5TIVvvcwtM"
        let id = "NP669OkoXo"
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
