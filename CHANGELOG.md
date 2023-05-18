# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1] - 18.05.2023

### Added

- Added support of http_proxy-like variables (`http_proxy`, `https_proxy`, `HTTPS_PROXY`, `all_proxy`, ...)
- Added `LAUNCHER_REQUESTS_TIMEOUT` environment variable

### Changed

- Now launcher applies selected session before launching the game.
  This should fix issues when you switch wine prefix, for example, by downloading a proton build

### Fixed

- Added a workaround for `7z` binary which fixes game updating issue

## [1.1.0] - 06.05.2023

### Added

- Added rules approving dialog to the first run window
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

[unreleased]: https://github.com/an-anime-team/honkers-launcher/compare/1.1.1...next
[1.1.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.1.0...1.1.1
[1.1.0]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.2...1.1.0
[1.0.2]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/an-anime-team/honkers-launcher/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/an-anime-team/honkers-launcher/releases/tag/1.0.0
