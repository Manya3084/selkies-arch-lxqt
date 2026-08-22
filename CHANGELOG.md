# Changelog

All notable changes to **selkies-arch-lxqt** are documented here.

Format inspired by [Keep a Changelog](https://keepachangelog.com/). Versioning follows the image/desktop stack, not upstream Selkies releases.

## [0.3.5] — 2026-08-22

### Fixed

- PipeWire quantum is no longer hard-coded to `128/48000` (xruns under GPU encode, audio pops). `PIPEWIRE_LATENCY` from compose is honored (default **512/48000**). Clock locked to 48 kHz / 512 in `pipewire-clock.conf`.

### Notes

- Rebuild required for the entrypoint/service scripts. Compose can still override: `PIPEWIRE_LATENCY: "1024/48000"` if pops remain.

## [0.3.4] — 2026-08-22

### Added

- **[multilib]** + **Steam** in the image: `steam`, `steam-devices`, `lib32-mesa` (32-bit OpenGL), `lib32-vulkan-intel` (32-bit ANV for Proton). The vulkan-intel pin stops pacman picking `lib32-vulkan-swrast`.
- Compose: `seccomp:unconfined` + `SYS_PTRACE` so Steam's pressure-vessel user namespaces work. Both Intel Vulkan ICDs (`intel_icd.json` + `intel_icd.i686.json`) so 32-bit games see Arc.

### Notes

- Rebuild required. Local compose should add `seccomp:unconfined` and the i686 ICD path; `VK_ICD_FILENAMES` with only the 64-bit JSON breaks Proton.
- Steam library lives in `./home` and survives rebuilds.

## [0.3.3] — 2026-08-22

### Fixed

- Regenerated `selkies-av1enc.patch` against current selkies `main` (`3a0bc7a`). The chroma grey-out hunk had a wrong line count (`malformed patch at line 59`), which aborted the Alpine `patch` during `docker compose build`.

### Notes

- Rebuild required (`docker compose build`). Keep your local `docker-compose.yml` (host cpuset / volumes / password).

## [0.3.2] — 2026-08-22

### Added

- **Chaotic-AUR + paru** on the 3020 image. Compose now passes `ENABLE_CHAOTIC_AUR=1` so the Dockerfile hook (already on main, previously off) installs the Chaotic keyring/mirrorlist and `paru` at build time. Extra `pacman-packages` can then pull Chaotic names.

### Notes

- Rebuild required (`docker compose build && docker compose up -d`). The running container does not pick this up.
- To skip: `docker compose build --build-arg ENABLE_CHAOTIC_AUR=0`.

## [0.3.1] — 2026-08-22

### Changed

- **pixelflux** is now built from the Selkies sibling [selkies-project/pixelflux](https://github.com/selkies-project/pixelflux) `main` (has `list_outputs` / `start_capture` / compositor socket). linuxserver/pixelflux `master` does not export those APIs.
- **pcmflux** is now built from [selkies-project/pcmflux](https://github.com/selkies-project/pcmflux) `main`.
- AV1 is a **real patch** (`addons/remotearch/patches/pixelflux-av1-vaapi.patch`) against that tree: `start_capture` with `output_mode=2` selects `av1_vaapi`. Removed the Dockerfile `pixelflux_wayland/` sed / NVENC-stub hacks (they never matched this layout) and the unused `pixelflux-nvenc-stub.rs`.
- Selkies still uses `addons/remotearch/patches/selkies-av1enc.patch` so `_get_capture_settings` sets `output_mode=2` for `av1enc`.

### Notes

- Do not set `PIXELFLUX_REPO` back to linuxserver/pixelflux unless you also drop the AV1 patch — it will not apply.
- Rebuild the image (`docker compose build`) after pulling.

## [0.3.0] — 2026-08-22

### Added

- **AV1 (VA-API)** on Intel Arc: pixelflux `master` is patched for `av1_vaapi` (`output_mode=2`); Selkies is patched so the dashboard lists **AV1 (VA-API Full Frame)** and the browser decodes `av01`.
- Default encoder remains H.264. WebRTC stays H.264.
- NVENC stub + ffmpeg-sys 9 so pixelflux builds on this CUDA-less Arch image.

### Notes

- Do not set `PIXELFLUX_REF=av1` — that old upstream branch has no compositor socket.
- Rebuild the image (`docker compose build`) after pulling; the running container does not pick this up.

## [0.2.0] — 2026-08-21

### Changed

- Image now **builds Selkies, pixelflux, and pcmflux from git** at `docker compose build` time (no extracted / vendored wheels).
  - Selkies: [selkies-project/selkies](https://github.com/selkies-project/selkies) `main` (wheel + web client)
  - pixelflux: [linuxserver/pixelflux](https://github.com/linuxserver/pixelflux) `master` (Rust, Arch stage)
  - pcmflux: [linuxserver/pcmflux](https://github.com/linuxserver/pcmflux) `master`
- Removed pre-built `addons/remotearch/wheels/*.whl` from the repo (directory kept with `.gitkeep` only)
- Compose defaults: `SELKIES_FROM_GIT=1`, `PIXELFLUX_FROM_GIT=1`, `PCMFLUX_FROM_GIT=1`

### Notes

- First build is long (Rust compile) and needs several GB of Docker disk.
- Pin refs with `--build-arg SELKIES_REF=` / `PIXELFLUX_REF=` / `PCMFLUX_REF=`.

## [0.1.2] — 2026-08-19

### Changed

- Default desktop user renamed from `ubuntu` to **`arch`** (uid/gid still 1000)
  - Home path is now `/home/arch`
  - Compose volume: `./home:/home/arch`
  - Runtime dir default: `/tmp/runtime-arch`
  - **Migration:** update your `docker-compose.yml` volume if you still mount `/home/ubuntu`. Existing `./home` data is fine (same uid).

### Added

- README section on mounting extra host directories (Games, ROMs, Media, …)

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
- Vendored wheels under `addons/remotearch/wheels/` (removed in 0.2.0)
- Joystick interposer and fake-udev sources (`addons/js-interposer`, `addons/fake-udev`)
- s6 service set: dbus, pipewire, pipewire-pulse, wireplumber, wayland, xvfb, lxqt, selkies, coturn
- Optional `dolphin-emu` package in the Dockerfile
- `.env.example` for `PASSWD`, host port, and GPU node selection (no secrets in compose)

### Security

- Desktop password comes from `${PASSWD}` / `.env` only — not hard-coded in `docker-compose.yml`
- Host-specific OMV bind mounts removed from the published compose file

### Credits

- Streaming stack: [selkies-project/selkies](https://github.com/selkies-project/selkies) (MPL-2.0)
- Desktop packaging and Intel Arc compose defaults: this repository
