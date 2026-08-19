# Selkies HTML5 web client

Copied into the image at `/opt/selkies-web` and selected with `SELKIES_WEB_ROOT`.

The vendored `selkies` wheel does **not** include `selkies.selkies_web`. Populate this directory from a working Selkies container before building:

```bash
# Example (paths on OMV)
WORKDIR=/srv/dev-disk-by-uuid-63896276-89b7-4ac3-b87a-74716c1ce7d9/DockerApps/selkies-arch-lxqt

docker cp remotearch-remotearch-1:/opt/selkies-web/. \
  "$WORKDIR/addons/remotearch/selkies-web/"

# Must contain at least:
#   index.html
#   src/selkies-core.js

ls -la "$WORKDIR/addons/remotearch/selkies-web/"
```

Then:

```bash
cd "$WORKDIR"
docker compose build
docker compose up -d
```

Commit the web files to git if you want clones to build offline without the copy step.
