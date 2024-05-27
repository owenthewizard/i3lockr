# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Update clap to v4
- Update itertools to 0.13
- Accept arguments for disabled features.
  Previously, a runtime error was returned.

### Added
- Add clippy and fmt to CI

### Removed
- `suggestions` and `color` features
- custom `--version` output

## [1.2.1] - 2024-03-15
## Changed
- Update dependencies (#20)
- No longer distribute SHA256SUMS, only signature
- Update signing key

## [1.2.0] - 2022-06-19
### Fixed
- Missing some `unsafe` keywords.
- Security issues in dependencies.

### Changed
- Screenshot is now captured with [scrap](https://github.com/owenthewizard/scrap).
- Arguments take `NonZero` types where appropriate.
- Image operations have been broken out into separate units.
- Make use of [imgref](https://crates.io/crates/imgref) instead of custom structures.
- Make use of [blend-srgb](https://crates.io/crates/blend-srgb) instead of manual blending.
- 100% Rust blur routine using [stackblur-iter](https://github.com/LoganDark/stackblur-iter) (resolves #13)

## [1.1.0] - 2020-02-10
### Changed
- Switch to `u32` rather than `[u8; 4]` for most operations
- Refactor into more separate compile units
- Switch from SysV SHM to Linux SHM

### Added
- Multithreaded brightness adjustment
- Nearest-neighbor scaling

## [1.0.0-final] - 2019-06-27
### Changed
- Complete rewrite with fewer dependencies
- Pass raw bytes to `i3lock` rather than encoding PNG

### Added
- Shell completions

## [0.1.2] - 2019-05-24
### Changed
- Don't `clone()` `args`, flag a `bool` instead
- Pass image to `i3lock` via `/dev/stdin` instead of temporay file

### Fixed
- Hacky `thread::sleep` solution no longer necessary

## [0.1.1] - 2019-03-30
### Changed
- i3lockr will wait on i3lock if i3lock is called with `--nofork`
- i3lockr will not wait on i3lock if i3lock is not called with `--nofork`
    - this fixes things like `i3lockr && systemctl suspend`

## [0.1.0] - 2019-01-12
### Added
- Initial release
