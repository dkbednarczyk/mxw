# AGENTS.md — mxw

CLI (Rust) that configures Glorious wireless mice (Model O/O-/D/D-/D2 Pro) over HID feature reports. Fork of korkje/mow; protocol was reverse-engineered from Glorious Core v1.

Output all commands, code and comments in English. It is preferable to respond to the user in the language they initiated the conversation in.

## Commands

- `make lint` = `cargo fmt` + `cargo clippy --all -- -D warnings`. Run it before finishing any change.
- `make build` = `cargo build --release`.
- CLI parsing can be verified without a device: bad values fail at clap parse time before HID is touched. Anything past parsing needs a real mouse.
- No push/PR CI exists; `.github/workflows/release.yml` only fires on `v*` tags and creates a GitHub release. Releases are plain Cargo.toml version bumps (see CHANGELOG.md).

## Gotchas

- On Linux, hidraw access needs udev permission: `70-glorious-mxw.rules` (repo root, vendor 0x258A product table) goes in `/etc/udev/rules.d/`; otherwise you'll hit "Permission denied" on `/dev/hidraw*` without elevated perms.
- Nix build (`flake.nix`, naersk) needs `pkg-config` + `libudev-zero`; Linux HID builds depend on libudev via the `hidapi` crate.
