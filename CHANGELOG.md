# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Unreleased

## Removed
- `--post-branding-api`, `--get-titles-by-video-id-api`, and `--get-thumbnails-by-video-id-api` in favor of
  `--main-api` (for APIs provided by SponsorBlockServer or compatible) and `--browser-api` (for APIs provided by
  DeArrowBrowser or compatible).

This is a breaking change.

## Added
- `--main-api` and `--browser-api` arguments: see above.
- `user <USER_ID> warnings (issued | received) [--newest <NEWEST>]` subcommand.
  - This will display the warnings issued or received by a specific SponsorBlock or DeArrow user.
  - It uses DeArrowBrowser's **internal** API.

## Changed
- `view` and `vote` subcommands no longer take the video ID as a flag (like `--video <VIDEO_ID>` or `-v <VIDEO_ID>`).
  Instead, they just take it as a regular argument: `vote <VIDEO_ID> title "Example"`.

This is a breaking change.

## [3.4.1] – 2024-11-17

### Changed
- Updated other dependencies.

## [3.4.0] – 2024-11-17

### Changed
- Updated to the latest version of DeArrowBrowser's internal API crate.

## [3.3.0] - 2024-02-04

### Changed
- Batch submission feature now interprets input file as CSV.

## [3.2.0] - 2024-01-16

### Added
- Batch submissions: intentionally left undocumented.

### Changed
- Minor changes to table layout when using the `view` subcommand.

## [3.1.0] - 2024-01-14
Initial release.

[3.4.1]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.4.1
[3.4.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.4.0
[3.3.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.3.0
[3.2.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.2.0
[3.1.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.1.0
