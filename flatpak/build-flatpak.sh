#!/usr/bin/env bash
# ==============================================================================
# LineCounter Local Flatpak Builder & Installer
# Fast, offline local Flatpak build for testing without downloading heavy SDKs.
# ==============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd)"

echo "=== LineCounter Flatpak Local Build & Install ==="

cd "${ROOT_DIR}"

# 1. Ensure latest release binary is built
echo "Building LineCounter desktop binary..."
cd "${ROOT_DIR}/src-tauri"
cargo build --release
cp target/release/linecounter "${ROOT_DIR}/linecounter-desktop"
cd "${ROOT_DIR}"

if [ ! -f "${ROOT_DIR}/linecounter" ]; then
    echo "Building LineCounter CLI binary..."
    cargo build --release --manifest-path "${ROOT_DIR}/Cargo.toml"
    cp target/release/linecounter "${ROOT_DIR}/linecounter"
fi

BUILD_DIR="${SCRIPT_DIR}/build"
REPO_DIR="${SCRIPT_DIR}/repo"

rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}" "${REPO_DIR}"

# 2. Initialize Flatpak directory using installed GNOME Platform 50
echo "Initializing container sandbox (org.gnome.Platform 50)..."
flatpak build-init "${BUILD_DIR}" com.linecounter.desktop org.gnome.Platform org.gnome.Platform 50

# 3. Install binaries, icons, and metadata
echo "Packaging application files..."
flatpak build "${BUILD_DIR}" install -D -m755 "${ROOT_DIR}/linecounter-desktop" /app/bin/linecounter-desktop
flatpak build "${BUILD_DIR}" install -D -m755 "${ROOT_DIR}/linecounter" /app/bin/linecounter
flatpak build "${BUILD_DIR}" install -D -m644 "${ROOT_DIR}/linecounter.png" /app/share/icons/hicolor/256x256/apps/com.linecounter.desktop.png
flatpak build "${BUILD_DIR}" install -D -m644 "${ROOT_DIR}/linecounter.svg" /app/share/icons/hicolor/scalable/apps/com.linecounter.desktop.svg
flatpak build "${BUILD_DIR}" install -D -m644 "${SCRIPT_DIR}/com.linecounter.desktop.desktop" /app/share/applications/com.linecounter.desktop.desktop
flatpak build "${BUILD_DIR}" install -D -m644 "${SCRIPT_DIR}/com.linecounter.desktop.metainfo.xml" /app/share/metainfo/com.linecounter.desktop.metainfo.xml
flatpak build "${BUILD_DIR}" install -D -m644 "${SCRIPT_DIR}/com.linecounter.desktop.metainfo.xml" /app/share/appdata/com.linecounter.desktop.appdata.xml

# 4. Set sandbox permissions (Wayland, X11, IPC, GPU DRI, Host filesystem)
echo "Configuring sandbox permissions..."
flatpak build-finish "${BUILD_DIR}" \
    --command=linecounter-desktop \
    --share=ipc \
    --socket=fallback-x11 \
    --socket=wayland \
    --device=dri \
    --filesystem=host

# 5. Export to local repository
echo "Exporting build to local repository..."
flatpak build-export "${REPO_DIR}" "${BUILD_DIR}"
flatpak build-update-repo "${REPO_DIR}"

# 6. Add local user remote and install
echo "Installing com.linecounter.desktop locally for testing..."
flatpak --user remote-delete --force linecounter-local-repo 2>/dev/null || true
flatpak --user remote-add --no-gpg-verify linecounter-local-repo "${REPO_DIR}"
flatpak --user install -y --reinstall linecounter-local-repo com.linecounter.desktop

# 7. Clean up temporary build and export directories to keep workspace lean
rm -rf "${BUILD_DIR}" "${REPO_DIR}"

echo ""
echo "============================================================"
echo "✓ LineCounter Flatpak installed successfully!"
echo "You can launch your local Flatpak app now using:"
echo "   flatpak run com.linecounter.desktop"
echo "============================================================"
