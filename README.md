<img src="repository/pics/launcher-main.png">

<img src="repository/pics/launcher-settings.png">

<br>

<p align="center">You could also try <a href="https://github.com/an-anime-team/honkers-launcher-gtk">the GTK branch</a></p>

<br>

# Requirements

To make this launcher work you need the following dependencies

| Name | Description |
| --- | --- |
| webkit2gtk | To run Neutralino apps |
| unzip | To unpack zip archives (DXVK / Wine) |
| tar | To unpack tar archives (DXVK / Wine) |
| p7zip | To unpack 7z archives (Game) |
| git | To check for new versions of the launcher |
| curl | For archive downloads: game, voice data, runners and so on |
| cabextract | To install fonts to the Wine prefix |
| libnotify | To send system notifications |
| samba | For Authentication |

## Install

### apt

```sh
sudo apt install unzip tar git curl samba p7zip-full cabextract libnotify-bin libayatana-appindicator3-1
```

### pacman

```sh
sudo pacman -Syu unzip tar git curl samba p7zip cabextract libnotify
```

### dnf

```sh
sudo dnf install unzip tar git curl samba p7zip p7zip-plugins cabextract libnotify webkit2gtk3 libappindicator
```

# Development

## Download source

```sh
git clone https://github.com/an-anime-team/honkers-launcher
cd honkers-launcher
npm i
npm run neu update
```

## Run

```sh
npm run dev
```

## Build

```sh
npm run build
```

## Bundle to AppImage

```sh
npm run bundle
```
