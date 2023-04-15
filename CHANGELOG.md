# Changelog
All notable changes to this project will be documented in this file.

## [0.3.2] - 2023-04-15

### Documentation

- Add in-the-media section (https://youtu.be/v3q707TRIIs)
- Update MSRV to 1.64.0

### Features

- Generate SBOM attestation for the Docker image
- Scan the Docker image using Syft

### Miscellaneous Tasks

- Switch to ratatui for tui rendering
- Switch to dtolnay/rust-toolchain action
- Update runner versions
- Update funding options
- Check dependency updates daily
- Bump dependencies

### Refactor

- Use parseit as a workspace dependency

## [0.3.1] - 2023-01-28
### Bug Fixes
- Apply clippy suggestions

### Documentation
- Fix badge links
- Update copyright years

### Miscellaneous Tasks
- Bump dependencies

## [0.3.0] - 2022-09-17
### Features
- (tui) Support saving changed values ([#13](https://github.com/orhun/git-cliff/issues/13))
  - Press <kbd>s</kbd> to set a parameter value and also save it to a file.
  - See https://github.com/orhun/systeroid#saving-values
- (tui) Make help list selection functional
  - Press <kbd>enter</kbd> on help menu to run the selected command.

### Miscellaneous Tasks
- Bump dependencies

## [0.2.2] - 2022-09-06
### Features
- Support listing parameters by subsection ([#44](https://github.com/orhun/systeroid/issues/44))

## [0.2.1] - 2022-08-30
### Updated
- Update the allowed licenses for cargo-deny
- Add metadata for cargo-binstall
- Remove unnecessary dirs dependency
- Bump dependencies

## [0.2.0] - 2022-08-11
### Features
- Add a configuration file ([#12](https://github.com/orhun/systeroid/issues/12))
  - See [configuration](https://github.com/orhun/systeroid#configuration) and [`systeroid.conf`](https://github.com/orhun/systeroid/blob/main/config/systeroid.conf)
- (tui) Show deprecated values optionally via `--deprecated` flag

### Documentation
- Update broken links

### Miscellaneous Tasks
- Update MSRV to 1.57.0
- Switch to Rust stable builds
- Bump dependencies
- Enable [GitHub Sponsors](https://github.com/sponsors/orhun) for funding
  - Consider supporting my open source work 💖

## [0.1.1] - 2022-04-19
### Added
- (cli) Support explaining multiple parameters
- Add installation instructions for Arch Linux

### Fixed
- (tui) Replace tab with whitespace in values

## [0.1.0] - 2022-04-16
Initial release.
