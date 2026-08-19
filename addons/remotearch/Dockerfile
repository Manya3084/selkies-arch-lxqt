# check=skip=SecretsUsedInArgOrEnv,UndefinedVar
#
# remotearch — Arch + LXQt (Intel Arc)
# MPL-2.0 — https://mozilla.org/MPL/2.0/

ARG PYPI_PACKAGE=selkies
ARG S6_OVERLAY_VERSION=3.2.1.0

# Joystick interposer + fake-udev
FROM archlinux:latest AS add-ons
RUN pacman -Syu --noconfirm --needed base-devel gcc && \
    pacman -Scc --noconfirm || true
COPY addons/js-interposer /build/js-interposer
COPY addons/fake-udev /build/fake-udev
RUN set -e; \
    mkdir -p /opt/add-ons/lib; \
    gcc -shared -fPIC -o /opt/add-ons/lib/selkies_joystick_interposer.so \
      /build/js-interposer/joystick_interposer.c -ldl -pthread; \
    make -C /build/fake-udev; \
    cp -a /build/fake-udev/libudev.so.1.0.0-fake \
          /build/fake-udev/libudev.so.1 \
          /build/fake-udev/libudev.so \
          /opt/add-ons/lib/ 2>/dev/null || \
      cp -a /build/fake-udev/libudev*.so* /opt/add-ons/lib/

# Main image
FROM archlinux:latest

LABEL org.opencontainers.image.title="selkies-arch-lxqt"
LABEL org.opencontainers.image.description="Selkies + Arch + LXQt (Intel Arc)"
LABEL org.opencontainers.image.source="https://github.com/Manya3084/selkies-arch-lxqt"
LABEL org.opencontainers.image.licenses="MPL-2.0"

ARG PYPI_PACKAGE
ARG S6_OVERLAY_VERSION
ARG TZ=UTC
ENV PASSWD=mypasswd

# Base packages (Arch Xvfb — do not COPY Debian Xvfb)
RUN pacman -Syu --noconfirm --needed \
        base ca-certificates curl jq which \
        sudo fakeroot fuse3 kmod tzdata \
        dbus shadow \
        xorg-server-xvfb xorg-xauth xorg-xkbcomp xorg-setxkbmap \
        xorg-xdpyinfo xorg-xrandr xorg-xprop xorg-xrdb wmctrl \
        libx11 libxcb libxkbcommon libxdamage libxfixes libxtst libxext \
        wayland xorg-xwayland \
        pipewire pipewire-pulse wireplumber libpulse \
        lxqt lxqt-session lxqt-panel lxqt-runner lxqt-config \
        openbox qterminal pcmanfm-qt xterm xdg-utils \
        mesa libva-mesa-driver vulkan-intel intel-media-driver \
        libva-utils mesa-utils vulkan-tools \
        python python-pip linux-api-headers \
        coturn labwc qt6-wayland \
        ttf-dejavu ttf-liberation noto-fonts noto-fonts-cjk noto-fonts-emoji \
        git \
    && pacman -Scc --noconfirm

# s6 supervision via s6-overlay (provides /command/s6-svscan)
RUN curl -fsSL -o /tmp/s6-overlay-noarch.tar.xz \
      "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz" && \
    curl -fsSL -o /tmp/s6-overlay-x86_64.tar.xz \
      "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz" && \
    tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && \
    tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz && \
    rm -f /tmp/s6-overlay-*.tar.xz && \
    ln -sf /command/s6-svscan /usr/bin/s6-svscan && \
    ln -sf /command/s6-svc /usr/bin/s6-svc 2>/dev/null || true && \
    ln -sf /command/s6-svstat /usr/bin/s6-svstat 2>/dev/null || true

RUN ln -snf "/usr/share/zoneinfo/${TZ}" /etc/localtime && echo "${TZ}" > /etc/timezone && \
    echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen || true

ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US:en"
ENV LC_ALL="en_US.UTF-8"

RUN groupadd -g 1000 arch 2>/dev/null || true; \
    useradd -ms /bin/bash -u 1000 -g 1000 arch 2>/dev/null || true; \
    usermod -aG wheel,audio,video,render,input,storage arch 2>/dev/null || \
      usermod -aG audio,video,input arch 2>/dev/null || true; \
    mkdir -p /etc/sudoers.d && \
    echo "arch ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/arch && \
    chmod 440 /etc/sudoers.d/arch && \
    echo "arch:${PASSWD}" | chpasswd

# LXQt defaults
RUN mkdir -p /etc/xdg/lxqt /etc/xdg/pcmanfm-qt/lxqt && \
    ICON_THEME=hicolor; \
    [ -d /usr/share/icons/Adwaita ] && ICON_THEME=Adwaita; \
    [ -d /usr/share/icons/breeze ] && ICON_THEME=breeze; \
    THEME=system; \
    [ -d /usr/share/lxqt/themes/ambiance ] && THEME=ambiance; \
    TERMINAL=xterm; \
    command -v qterminal >/dev/null 2>&1 && TERMINAL=qterminal; \
    echo "LXQt defaults: icon_theme=${ICON_THEME} theme=${THEME} terminal=${TERMINAL}"; \
    printf '[General]\nicon_theme=%s\ntheme=%s\n' "${ICON_THEME}" "${THEME}" > /etc/xdg/lxqt/lxqt.conf; \
    printf '[Desktop]\nWallpaperMode=color\nBgColor=#2e3436\nFgColor=#ffffff\nShowHome=true\nShowTrash=false\nShowComputer=true\nShowNetwork=false\n[System]\nTerminal=%s\n' \
        "${TERMINAL}" > /etc/xdg/pcmanfm-qt/lxqt/settings.conf; \
    printf 'TerminalEmulator=%s.desktop\n' "${TERMINAL}" > /etc/xdg/qtxdg.conf

# Selkies from wheels/ or PyPI
# Prefer a single selkies wheel: .dev0 first, else any selkies-*.whl, else PyPI.
# Web UI is NOT in this wheel — see COPY of selkies-web below.
COPY addons/remotearch/wheels/ /tmp/wheels/
RUN pacman -Sy --noconfirm --needed \
      python python-pip python-evdev \
      base-devel linux-api-headers && \
    set -e; \
    SELKIES_WHL="$(ls -1 /tmp/wheels/selkies-0.0.0.dev0-*.whl 2>/dev/null | head -1 || true)"; \
    if [ -z "${SELKIES_WHL}" ]; then \
      SELKIES_WHL="$(ls -1 /tmp/wheels/selkies-*.whl 2>/dev/null | head -1 || true)"; \
    fi; \
    if [ -z "${SELKIES_WHL}" ]; then \
      PKGS="${PYPI_PACKAGE}"; \
    else \
      PKGS="${SELKIES_WHL}"; \
      for dep in pixelflux pcmflux; do \
        W="$(ls -1 /tmp/wheels/${dep}-*.whl 2>/dev/null | head -1 || true)"; \
        [ -n "${W}" ] && PKGS="${PKGS} ${W}"; \
      done; \
    fi; \
    echo "pip install: ${PKGS}"; \
    python -m pip install --break-system-packages --no-cache-dir \
      --retries 5 --timeout 60 --force-reinstall ${PKGS}; \
    (command -v selkies-resize >/dev/null || \
      find /usr -name 'selkies-resize' 2>/dev/null | head -1 | \
      xargs -r -I{} ln -sf {} /usr/local/bin/selkies-resize) || true; \
    rm -rf /tmp/wheels

COPY --from=add-ons /opt/add-ons/lib/ /usr/lib/

COPY --chown=1000:1000 addons/remotearch/labwc-rc.xml /etc/xdg/labwc/rc.xml
COPY --chown=1000:1000 addons/remotearch/container-entrypoint.sh /etc/container-entrypoint.sh
COPY --chown=1000:1000 addons/remotearch/selkies-entrypoint.sh /etc/selkies-entrypoint.sh
COPY --chown=1000:1000 addons/remotearch/services/ /etc/service/
# HTML5 client (not inside the vendored wheel — see addons/remotearch/selkies-web/README.md)
COPY addons/remotearch/selkies-web/ /opt/selkies-web/
RUN chmod -f 755 /etc/container-entrypoint.sh /etc/selkies-entrypoint.sh && \
    find /etc/service \( -name run -o -name finish \) -exec chmod -f 755 {} + && \
    chmod 4755 /usr/bin/sudo 2>/dev/null || true

RUN echo "[ ! -z \"\$TERM\" -a -r /etc/motd ] && cat /etc/motd" >> /etc/bash.bashrc && \
    printf "==============================================\n=         remotearch (Arch + LXQt)            =\n==============================================\n" > /etc/motd

# Extra apps — add packages here so they survive image rebuilds
RUN pacman -Sy --noconfirm --needed \
      dolphin-emu \
      firefox \
      gvfs gvfs-smb \
    && pacman -Scc --noconfirm

USER 1000

ENV PIPEWIRE_LATENCY="128/48000"
ENV XDG_RUNTIME_DIR=/tmp/runtime-arch
ENV PIPEWIRE_RUNTIME_DIR="${XDG_RUNTIME_DIR}"
ENV PULSE_RUNTIME_PATH="${XDG_RUNTIME_DIR}/pulse"
ENV PULSE_SERVER="unix:${XDG_RUNTIME_DIR}/pulse/native"
ENV DBUS_SYSTEM_BUS_ADDRESS="unix:path=${XDG_RUNTIME_DIR}/dbus-system-bus"
ENV SELKIES_WAYLAND_COMPOSITOR=labwc
ENV SELKIES_WAYLAND_OUTPUTS=""
ENV SELKIES_WAYLAND_X11_FALLBACK=true
ENV START_LXQT=true
ENV SELKIES_WEB_ROOT=/opt/selkies-web
ENV SELKIES_PORT=8080
ENV SHELL=/bin/bash
ENV USER=arch
ENV HOME=/home/arch
WORKDIR /home/arch

EXPOSE 8080
ENTRYPOINT ["/etc/container-entrypoint.sh"]
