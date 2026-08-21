# selkies-arch-lxqt

Arch Linux + LXQt desktop streamed in a browser via [Selkies](https://github.com/selkies-project/selkies).

One image, one `docker compose build`. Aimed at Intel Arc (A750/A770) but works with AMD (Mesa RADV) and NVIDIA (with host drivers + NVIDIA Container Toolkit).

**Current release:** [0.2.0](CHANGELOG.md) (2026-08-21)

## Features

- Arch Linux base, LXQt session, labwc (Wayland) + XWayland
- Selkies WebSocket streamer (HTML5 client)
- PipeWire audio
- s6-overlay process supervision
- Joystick interposer + fake-udev (from Selkies addons)
- Optional apps in the Dockerfile (`dolphin-emu`, `firefox`, `gvfs`)
- Extra pacman packages from a **persistent list** (reinstalled on each start, no Dockerfile edit)
- **Built from git at image build time** — [Selkies](https://github.com/selkies-project/selkies) `main`, [pixelflux](https://github.com/linuxserver/pixelflux) `master`, [pcmflux](https://github.com/linuxserver/pcmflux) `master` (no vendored wheels)

## Quick start

```bash
git clone https://github.com/Manya3084/selkies-arch-lxqt.git
cd selkies-arch-lxqt
cp .env.example .env   # set PASSWD + GPU vars if needed
mkdir -p home
docker compose build   # first build is long (Rust + Selkies web)
docker compose up -d
```

Open `http://<host>:3020` and log in with the password from `.env`.

The first image build compiles **pixelflux** and **pcmflux** (Rust) and builds Selkies + its web client from git. Expect several minutes and a few GB of Docker disk. Later rebuilds are faster if Docker layer cache is intact. Use `docker compose build --no-cache` to force a fresh git fetch + Arch `pacman -Syu`.

## Compose layout

```yaml
build:
  context: .
  dockerfile: addons/remotearch/Dockerfile
  args:
    SELKIES_FROM_GIT: "1"
    SELKIES_REF: "main"
    PIXELFLUX_FROM_GIT: "1"
    PIXELFLUX_REF: "master"
    PCMFLUX_FROM_GIT: "1"
    PCMFLUX_REF: "master"
```

- **context** must be the repo root (so `COPY addons/js-interposer` works)
- **dockerfile** is the Arch image (`addons/remotearch/Dockerfile`)

### Build args

| Arg | Default (Dockerfile) | Compose | Meaning |
|---|---|---|---|
| `SELKIES_FROM_GIT` | `0` | `1` | Build Selkies wheel + web from git |
| `SELKIES_REPO` | `https://github.com/selkies-project/selkies.git` | | Upstream (or your fork) |
| `SELKIES_REF` | `main` | `main` | Branch or commit |
| `PIXELFLUX_FROM_GIT` | `0` | `1` | Build pixelflux from git (Rust) |
| `PIXELFLUX_REPO` | `https://github.com/linuxserver/pixelflux.git` | | |
| `PIXELFLUX_REF` | `master` | `master` | |
| `PCMFLUX_FROM_GIT` | `0` | `1` | Build pcmflux from git (Rust) |
| `PCMFLUX_REPO` | `https://github.com/linuxserver/pcmflux.git` | | |
| `PCMFLUX_REF` | `master` | `master` | |

Pin a commit instead of a floating branch:

```bash
docker compose build --build-arg SELKIES_REF=<sha> \
  --build-arg PIXELFLUX_REF=<sha> --build-arg PCMFLUX_REF=<sha>
```

Point `PIXELFLUX_REPO` / `SELKIES_REPO` at a **fork** when carrying patches.

### AV1 (Intel Arc / VA-API)

The `test/av1` image patches **pixelflux master** to encode AV1 via `av1_vaapi`
(`output_mode=2`) and patches **Selkies** so the encoder menu includes `av1enc`.
Default encoder stays H.264 so a machine without AV1 encode still streams.
In the dashboard pick **AV1 (VA-API Full Frame)** after the desktop is up.
Needs Intel Arc (or other VA-API AV1 encode) on `/dev/dri`. WebRTC stays H.264.

## GPU setup

Find your render node and PCI ID on the host:

```bash
ls -l /dev/dri/by-path/
lspci -nn | grep -E 'VGA|3D|Display'
```

Defaults in `docker-compose.yml` / `.env`:

| Variable | Meaning |
|---|---|
| `DRI_NODE` | Render node, e.g. `/dev/dri/renderD128` |
| `MESA_VK_DEVICE_SELECT` | `vendor:device` PCI ID (Mesa device picker) |
| `VK_ICD_FILENAMES` | Vulkan ICD JSON inside the image |

### Intel Arc (default)

Image packages: `vulkan-intel`, `intel-media-driver`, Mesa.

| Card | `MESA_VK_DEVICE_SELECT` | Notes |
|---|---|---|
| Arc A750 | `8086:56a1` | Default in compose |
| Arc A770 | `8086:56a0` | |
| Arc A580 | `8086:56a2` | |
| Arc B580 | `8086:e20b` | Battlemage — confirm with `lspci -nn` |

```bash
# .env
DRI_NODE=/dev/dri/renderD128
MESA_VK_DEVICE_SELECT=8086:56a1
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/intel_icd.json
```

### AMD (Mesa RADV)

Works with the same `/dev/dri` pass-through as Intel. Image already has Mesa; set the ICD to the Radeon one and your GPU’s PCI ID.

| Card (examples) | Typical PCI ID |
|---|---|
| RX 7900 XTX / XT | `1002:744c` / `1002:7448` |
| RX 7800 XT | `1002:747e` |
| RX 7600 | `1002:7480` |
| RX 6800 XT | `1002:73bf` |
| RX 6700 XT | `1002:73df` |
| RX 6600 / XT | `1002:73ff` / `1002:73df` |
| RX 580 | `1002:67df` |

Always confirm with `lspci -nn`.

```bash
# .env
DRI_NODE=/dev/dri/renderD128
MESA_VK_DEVICE_SELECT=1002:744c
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/radeon_icd.x86_64.json
```

If the ICD path differs inside the container, check with:

```bash
docker exec <container> ls /usr/share/vulkan/icd.d/
```

### NVIDIA

This image is **Mesa-oriented**. Proprietary NVIDIA support needs host drivers **and** the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html).

| Card (examples) | Typical PCI ID |
|---|---|
| RTX 4090 | `10de:2684` |
| RTX 4080 | `10de:2704` |
| RTX 4070 / Ti | `10de:2786` / `10de:2782` |
| RTX 3090 | `10de:2204` |
| RTX 3080 | `10de:2206` |
| RTX 3060 | `10de:2504` |
| GTX 1080 | `10de:1b80` |

Confirm with `lspci -nn`.

**1. Host requirements**

- Proprietary NVIDIA driver installed and working
- `nvidia-container-toolkit` installed
- Docker configured to use the NVIDIA runtime (toolkit install usually does this)

**2. Compose changes** (add or replace the GPU-related bits)

```yaml
services:
  remotearch:
    # ... existing build/image/ports/volumes ...
    environment:
      # ... existing env ...
      NVIDIA_VISIBLE_DEVICES: all
      NVIDIA_DRIVER_CAPABILITIES: all
      # Optional if using Mesa device select against a specific card:
      # MESA_VK_DEVICE_SELECT: 10de:2684
      VK_ICD_FILENAMES: /usr/share/vulkan/icd.d/nvidia_icd.json
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
    # Keep /dev/dri if you also want KMS/render nodes; primary path is the NVIDIA runtime
    devices:
      - /dev/dri:/dev/dri
      - /dev/fuse:/dev/fuse
```

Alternatively, with an older toolkit setup:

```yaml
runtime: nvidia
```

**3. ICD inside the image**

The stock image may not ship `nvidia_icd.json`. On NVIDIA hosts you typically rely on the toolkit mounting driver userspace, or you install matching userspace in a custom layer. If Vulkan fails to load, check:

```bash
docker exec <container> ls /usr/share/vulkan/icd.d/
docker exec <container> nvidia-smi   # should work if toolkit + runtime are correct
```

**Note:** Pure Mesa paths (`/dev/dri` only, no toolkit) do **not** use proprietary NVIDIA drivers. For NVIDIA, prefer the toolkit + `deploy.resources` / `runtime: nvidia` approach above.

### Multiple GPUs

Pin the node and PCI ID so Selkies and apps hit the card you want:

```bash
ls -l /dev/dri/by-path/
# e.g. pci-0000:03:00.0-render -> ../renderD129

# .env
DRI_NODE=/dev/dri/renderD129
MESA_VK_DEVICE_SELECT=8086:56a1   # or 1002:… / 10de:…
```

## Adding packages

`pacman -S` inside a running container is **lost on the next image rebuild** (`/usr` is not a volume). Use one of these instead.

### Persistent extra list (no Dockerfile edit)

On the host (lives in `./home`, already bind-mounted):

```bash
mkdir -p home/.config
cat >> home/.config/pacman-packages << 'EOF'
mpv
htop
EOF
```

Or from a terminal **inside** the desktop:

```bash
mkdir -p ~/.config
echo mpv >> ~/.config/pacman-packages
```

Restart the container (`docker compose up -d`). The entrypoint runs `pacman -Sy --needed` for that list before the desktop starts. Already-installed packages are skipped. Needs network on start.

You can also set compose env (space-separated):

```yaml
environment:
  PACMAN_PACKAGES: "mpv htop"
```

Example file: [`addons/remotearch/pacman-packages.example`](addons/remotearch/pacman-packages.example).

**This is not AUR.** Official repos only. AppImages dropped in `/home/arch` already persist.

### Baked into the image

For packages everyone should get, edit the “Extra apps” block in `addons/remotearch/Dockerfile`:

```dockerfile
RUN pacman -Sy --noconfirm --needed \
      dolphin-emu \
      firefox \
      gvfs gvfs-smb \
    && pacman -Scc --noconfirm
```

Then `docker compose build && docker compose up -d`.

## Persist home and extra directories

The compose file bind-mounts a host folder over the container home so settings and installs survive rebuilds:

```yaml
volumes:
  - ./home:/home/arch
```

Add more host directories the same way (ROMs, games, media, downloads, …). Paths on the left are on the **host**; paths on the right are inside the container:

```yaml
volumes:
  - ./home:/home/arch
  - /path/on/host/Games:/home/arch/Games
  - /path/on/host/ROMs:/home/arch/ROMs
  - /mnt/storage/Media:/home/arch/Media
```

Notes:

- Create the host paths first (`mkdir -p …`) or Docker may create them as root.
- Container user is `arch` (uid/gid **1000**). Host files should be readable/writable by uid 1000, e.g. `chown -R 1000:1000 /path/on/host/Games`.
- Do not commit `home/` or `.env`.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Selkies components and derived scripts are under the [Mozilla Public License 2.0](LICENSE).  
Arch packages follow their own licenses.
