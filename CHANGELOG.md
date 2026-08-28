# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
While the project is pre-1.0, the **minor** version is bumped for new commands or
flags and for user-visible behavior changes, and the **patch** version is bumped
for bug fixes and internal changes.

## [0.3.0] - 2026-08-28

### Added

- `config dpi-stages`, `config dpi-colors`, and `config dpi-stage` now support
  profiles with 1 to 6 DPI stages instead of a hard-coded 4. (#22)
- `config dpi-stages --uniform <DPI>` sets every stage in the profile to the
  same resolution, keeping the profile's current stage count.

### Changed

- `config dpi-stages` resets the active DPI stage to the first one whenever the
  stage list changes, mirroring Glorious Core. Previously the active stage was
  left untouched.
- `config dpi-colors` now requires exactly one color per configured DPI stage
  (previously it always expected four).
- The DPI-stages packet length is derived from the stage count rather than
  hard-coded, so shorter and longer stage lists are framed correctly.

### Fixed

- `config dpi-stages` no longer reads the device before writing, so a flaky or
  out-of-range stage-count read can no longer abort the command that would fix
  it.

## [0.2.4] - 2026-08-27

### Fixed

- `report dpi` failing with "no response to read command 0x81" on mice that are
  not configured with exactly four DPI stages.

### Changed

- `report firmware` shares the common `report::read` request/retry path.
- Dropped a trailing space from `report battery --hide-status` output.

## [0.2.3] - 2026-08-27

### Added

- `report profile` prints the active profile id.
- `report dpi` prints the active DPI stage and its resolution, with `--all`,
  `--dpi`, `--stage`, and `--profile` flags.
- udev rules file for rootless use, packaged as the `mxw-udev` AUR package.

### Changed

- Reworked CLI parsing and device matching.
- The tool now waits for the mouse to wake before applying any change.

## [0.2.2] - 2025-12-31

### Added

- `report battery --hide-status` prints the percentage without the charging
  state.

## [0.2.1] - 2025-10-04

### Changed

- Dependency updates; removed an unused internal module.

## [0.2.0] - 2025-09-23

### Added

- Device codes for the Series One Pro and further mouse variants.

### Changed

- Error handling moved to `anyhow` throughout.

[0.3.0]: https://github.com/dkbednarczyk/mxw/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/dkbednarczyk/mxw/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/dkbednarczyk/mxw/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/dkbednarczyk/mxw/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/dkbednarczyk/mxw/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/dkbednarczyk/mxw/compare/v0.1.2...v0.2.0
