# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [4.1.0] - 2025-02-15

### Added
- For normal title and thumbnail votes, you can now pass the `--using-casual` flag to indicate that
  you use casual mode. This is saved alongside the submission.
- A new subcommand for `vote`: `dearrow-cli vote <VIDEO_ID> casual <CATEGORY>*` or `dearrow-cli vote <VIDEO_ID> --downvote casual`

## [4.0.0] - 2024-11-24

### Removed
- `--post-branding-api`, `--get-titles-by-video-id-api`, and `--get-thumbnails-by-video-id-api` in favor of
  `--main-api` (for APIs provided by SponsorBlockServer or compatible) and `--browser-api` (for APIs provided by
  DeArrowBrowser or compatible).

This is a breaking change.

### Added
- `--main-api` and `--browser-api` arguments: see above.
- `user <USER_ID> warnings (issued | received) [--newest <NEWEST>]` command.
  - This will display the warnings issued or received by a specific SponsorBlock or DeArrow user.
  - It uses DeArrowBrowser's **internal** API.
- `--was-warned` flag for votes on titles.
  - This will self-report the submission, used for when users dismiss an auto-warning in the actual addon.
- `o` flag in the `view` command to indicate an original title. It is not displayed for thumbnails, since they
  show "original" in the timestamp column.
- `main` argument to `view`, to see live submissions served by a SponsorBlockServer instance rather than DAB.
  - This shows less useful information: no removed, downvoted (with score <= -2) or shadowhidden submissions,
    no usernames, no "unverified user" flag, no VIP flag.
  - Usage: `view <VIDEO_ID> main`

### Changed
- the `view` and `vote` commands no longer take the video ID as a flag (like `--video <VIDEO_ID>` or `-v <VIDEO_ID>`).
  Instead, they just take it as a regular argument: `vote <VIDEO_ID> title "Example"`.
  - This is a breaking change.
- `batch` now uses stderr instead of stdout for user interactions.

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

[4.1.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v4.1.0
[4.0.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v4.0.0
[3.4.1]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.4.1
[3.4.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.4.0
[3.3.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.3.0
[3.2.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.2.0
[3.1.0]: https://github.com/mschae23/dearrow-cli/releases/tag/v3.1.0
