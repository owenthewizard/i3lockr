# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- 100% Rust blur routine (maybe)
- More multithreading? (hopefully)
- Drop `imagefmt` dependency (probably)
- Switch to `u32` rather than `[u8; 4]` (definitely)
- Refactoring into more separate compile units (probably)
- Switch from SysV SHM to Linux SHM (could also use POSIX I suppose...) (definitely)
- Make use of hugepages (probably)

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
