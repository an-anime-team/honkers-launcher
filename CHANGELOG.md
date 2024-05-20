# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.6.3] - 20.05.2024

### Fixed

- Updated background URI API key

## [1.6.2] - 08.05.2024

### Added

- Added Czech

### Changed

- Updated API links

## [1.6.1] - 24.03.2024

### Added

- Bundle `applications-system-symbolic` icon to the app
- Added "force grab cursor" option to the gamescope settings
- Added Thai
- Added Ukrainian

### Changed

- Update wish url
- Updated dependencies
- Improved app args parsing
- Updated locales

### Fixed

- Fixed GtkSwitch UI state representation

## [1.6.0] - 29.12.2023

### Added

- Added Vietnamese
- Added Korean
- Added Dutch
- Added new `Concerning` patch status
- Made free space checks resolve symlinks
- Added `UpdatingPermissions` installation step
- Downloaders now will skip finished files and truncate them if needed
- Added new fix for the API responses
- Added special tooltips for concerning patch status

### Fixed

- Fixed "Kill game process" button

### Changed

- Updated development libraries versions
- Updated Turkish
- Updated German
- Updated Polish
- Updated Chinese

## [1.5.0] - 20.08.2023

### Added

- Added feature to map wine drives
- Added `%launch_args%` magic word for game launching command.
  Now you can use `%bash_command% <script> %launch_args%` to run custom script
- Added `--session <name>` flag to switch active session
- Added Portuguese
- Added Polish

### Changed

- Updated Swedish
- Improved files migration code. In the best case scenarios, it will work immediately now
- Updated Chinese game API link

## [1.4.0] - 04.08.2023

### Added

- Added multi-region support

### Fixed

- Fixed logo size in the first run window
- Fixed 7z dependency check on some systems

### Changed

- Updated Turkish
- Updated Italian
- Updated Japanese
- Updated Turkish
- Updated Indonesian

## [1.3.0] - 02.08.2023

### Added

- Added new gamescope version compatibility
- Added "launcher behavior" option
- Added "kill game process" button when chosen behavior keeps launcher window open
- Bundled some icons into the app for consistency across the systems
- Added better panics handler
- Added Swedish

### Fixed

- Fixed jadeite patch state handling from the metadata file
- Fixed game pre-downloading button sensitivity when the update was partially downloaded, but then interrupted
- Fixed game pre-downloading button visibility when jadeite patch state is not "verified"

### Changed

- Improved pre-downloads state checking
- Replaced translation functions by `tr!` macro
- Reworked app resources structure

## [1.2.2] - 20.07.2023

### Fixed

- Fixed telemetry disabling
- Fixed jadeite patch state handling from the metadata file

### Changed

- Updated Italian
- Updated Hungarian
- Updated Japanese
- Updated Indonesian
- Updated Spanish
- Updated Turkish
- Updated Chinese

## [1.2.1] - 18.06.2023

### Added

- Added deletion of old patch files (just in case)
- Added telemetry disabling state support
- Added Discord RPC icons updating

### Changed

- Returned back old `background` file path

## [1.2.0] - 15.06.2023

### Added

- Added Italian
- Added Indonesian
- Added dynamic main button icon switching
- Set button label as "Resume" when the diff is part downloaded
- Added options to use wine / gstreamer shared libraries from selected wine build.
  These options will configure `LD_LIBRARY_PATH` and `GST_PLUGIN_PATH` environment variables
- Added setting of `LC_ALL` in wine lang setting
- Added Discord RPC icon selection
- Integrated Jadeite patch
- Added Japanese
- Added Hungarian

### Fixed

- Fixed progress bar style after running game repairer
- Fixed repair button functionality
- Fixed default launcher language selection at the first start
- Fixed some installer updates reporting (including "checking free space")
- Fixed check button style for newly made sessions
- Fixed repairer's NaN progress
- Fixed game session selection when current one is removed

### Changed

- Reworked game sessions selection
- Updated French
- Made initial tasks async which has decreased startup time
- Updated Spanish
- Improved 7z support

### Removed

- Removed patch mirror migration

## [1.1.1] - 18.05.2023

### Added

- Added support of http_proxy-like variables (`http_proxy`, `https_proxy`, `HTTPS_PROXY`, `all_proxy`, ...)
- Added `LAUNCHER_REQUESTS_TIMEOUT` environment variable

### Changed

- Now launcher applies selected session before launching the game.
  This should fix issues when you switch wine prefix, for example, by downloading a proton build
- Mfplat patch is now disabled by default 

### Fixed

- Added a workaround for `7z` binary which fixes game updating issue

## [1.1.0] - 06.05.2023

### Added

- Added game settings section
- Added game sessions manager
- Added `LAUNCHER_FOLDER` variable support.
  Using this you can specify root path where the launcher stores `config.json` and other files
- Added patch repository mirror

### Changed

- Improved launcher logo rendering quality
- Reworked entry rows in the settings

### Fixed

- Fixed sandboxed game running (sounds are broken for now)

## [1.0.2] - 18.04.2023

### Fixed

- Fixed mfplat patch applying. Added support for proton builds
- Fixed wine tools running using proton builds

## [1.0.1] - 17.04.2023

### Added

- Added mfplat patch integration
- Added arguments and symlinks editor to sandbox settings

### Fixed

- Fixed launcher pup-up on game launching
- Fixed game running issue if you have spaces in paths

## [1.0.0] - 16.04.2023

🚀 Initial release

<br>

[unreleased]: https://github.com/an-anime-team/honkers-launcher/compare/1.6.3...next
[1.6.3]: https://github.com/an-anime-team/honkers-launcher/compare/1.6.2...1.6.3
[1.6.2]: https://github.com/an-anime-team/honkers-launcher/compare/1.6.1...1.6.2
[1.6.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.6.0...1.6.1
[1.6.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.5.0...1.6.0
[1.5.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.4.0...1.5.0
[1.4.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.3.0...1.4.0
[1.3.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.2.2...1.3.0
[1.2.2]: https://github.com/an-anime-team/honkers-launcher/compare/1.2.1...1.2.2
[1.2.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.2.0...1.2.1
[1.2.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.1.1...1.2.0
[1.1.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.1.0...1.1.1
[1.1.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.2...1.1.0
[1.0.2]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/an-anime-team/honkers-launcher/releases/tag/1.0.0
