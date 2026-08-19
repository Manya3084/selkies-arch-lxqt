# Changelog

All notable changes to **selkies-arch-lxqt** are documented here.

Format inspired by [Keep a Changelog](https://keepachangelog.com/). Versioning follows the image/desktop stack, not upstream Selkies releases.

## [0.1.1] — 2026-08-19

### Fixed

- Dockerfile wheel install now prefers a single `selkies-0.0.0.dev0-*.whl` (bundled web client), then any other `selkies-*.whl`, then falls back to PyPI. Never passes multiple selkies wheels to `pip`.
- Removed `selkies-1.6.1` from `addons/remotearch/wheels/` so builds are deterministic for anyone cloning the repo.

## [0.1.0] — 2026-08-19

First public cut of a standalone **Arch Linux + LXQt** Selkies desktop image.

### Added

- Arch-based Dockerfile (`addons/remotearch/Dockerfile`) with LXQt, labwc, XWayland, PipeWire, and s6-overlay
- One-shot `docker compose` build (`context: .`, `dockerfile: addons/remotearch/Dockerfile`)
- Intel Arc-oriented GPU defaults (`DRI_NODE`, `MESA_VK_DEVICE_SELECT`, Vulkan ICD path)
- Vendored wheels under `addons/remotearch/wheels/`:
  - `selkies-0.0.0.dev0` (preferred local build with web client)
  - `pixelflux-2.0.0`, `pcmflux-2.0.0` (cp314 manylinux)
- Joystick interposer and fake-udev sources (`addons/js-interposer`, `addons/fake-udev`)
- s6 service set: dbus, pipewire, pipewire-pulse, wireplumber, wayland, xvfb, lxqt, selkies, coturn
- Optional `dolphin-emu` package in the image
- `.env.example` for `PASSWD`, host port, and GPU node selection (no secrets in compose)

### Security

- Desktop password comes from `${PASSWD}` / `.env` only — not hard-coded in `docker-compose.yml`
- Host-specific OMV bind mounts removed from the published compose file

### Notes

- Only one `selkies-*.whl` is needed in `wheels/` (the Dockerfile will pick `.dev0` first if present).
- Upstream Selkies `docs/`, `infra/`, and `scripts/` may still exist in the tree from the initial import; they are not required to build or run this desktop image and can be deleted in a later cleanup commit.

### Credits

- Streaming stack: [selkies-project/selkies](https://github.com/selkies-project/selkies) (MPL-2.0)
- Desktop packaging and Intel Arc compose defaults: this repository
