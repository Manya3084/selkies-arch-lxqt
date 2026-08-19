# selkies-arch-lxqt

Arch Linux + LXQt desktop streamed in a browser via [Selkies](https://github.com/selkies-project/selkies).

One image, one `docker compose build`. No Ubuntu extract step. Aimed at Intel Arc (A750/A770) but works with any `/dev/dri` render node.

## Features

- Arch Linux base, LXQt session, labwc (Wayland) + XWayland
- Selkies WebSocket streamer (HTML5 client)
- PipeWire audio
- s6-overlay process supervision
- Joystick interposer + fake-udev (from Selkies addons)
- Optional apps in the Dockerfile (currently `dolphin-emu`)
- Vendored Python wheels under `addons/remotearch/wheels/` for offline-friendly builds

## Quick start

```bash
git clone https://github.com/Manya3084/selkies-arch-lxqt.git
cd selkies-arch-lxqt
cp .env.example .env   # set PASSWD
mkdir -p home
docker compose build
docker compose up -d
```

Open `http://<host>:3020` and log in with the password from `.env`.

## Compose layout

```yaml
build:
  context: .
  dockerfile: addons/remotearch/Dockerfile
```

- **context** must be the repo root (so `COPY addons/js-interposer` works)
- **dockerfile** is the Arch image, not a Selkies wheel builder

## GPU (Intel Arc)

Defaults in `docker-compose.yml`:

| Variable | Meaning |
|---|---|
| `DRI_NODE` | `/dev/dri/renderD128` (change if needed) |
| `MESA_VK_DEVICE_SELECT` | `8086:56a1` = Arc A750; A770 is often `8086:56a0` |
| `VK_ICD_FILENAMES` | Intel Vulkan ICD path inside the image |

List nodes on the host:

```bash
ls -l /dev/dri/by-path/
```

## Wheels

`addons/remotearch/wheels/` should contain:

- `selkies-*.whl`
- `pixelflux-*.whl` (optional but recommended)
- `pcmflux-*.whl` (optional but recommended)

If no `selkies-*.whl` is present, the Dockerfile falls back to `pip install selkies` from PyPI.

Rebuild wheels from a working container:

```bash
docker exec <container> python -m pip wheel --no-deps -w /tmp/w selkies pixelflux pcmflux
docker cp <container>:/tmp/w/. ./addons/remotearch/wheels/
```

## Adding packages

Edit the "Extra apps" block near the end of `addons/remotearch/Dockerfile`:

```dockerfile
RUN pacman -Sy --noconfirm --needed \
      dolphin-emu \
      # firefox \
    && pacman -Scc --noconfirm
```

Then `docker compose build && docker compose up -d`.

## Persist the home directory

```yaml
volumes:
  - ./home:/home/ubuntu
```

Do not commit `home/` or `.env`.

## License

Selkies components and derived scripts are under the [Mozilla Public License 2.0](LICENSE).  
Arch packages follow their own licenses.
