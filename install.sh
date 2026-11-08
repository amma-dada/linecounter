#!/usr/bin/env bash
# ==============================================================================
# LineCounter Linux One-Click Installer
# Works across Ubuntu, Debian, Fedora, Arch Linux, openSUSE, Pop!_OS, Linux Mint,
# and all Desktop Environments (GNOME, KDE Plasma, COSMIC, Cinnamon, XFCE, MATE).
# ==============================================================================

set -e

SOURCE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

if [ "$1" = "--uninstall" ] || [ "$1" = "-u" ]; then
    echo "Uninstalling LineCounter..."
    rm -f "${HOME}/.local/bin/linecounter"
    rm -f "${HOME}/.local/share/applications/com.linecounter.desktop.desktop"
    rm -f "${HOME}/.local/share/applications/LineCounter.desktop"
    rm -f "${HOME}/Desktop/LineCounter.desktop"
    rm -f "${HOME}/.local/share/icons/hicolor/scalable/apps/com.linecounter.desktop.svg"
    rm -f "${HOME}/.local/share/icons/hicolor/scalable/apps/linecounter.svg"
    rm -f "${HOME}/.local/share/pixmaps/com.linecounter.desktop.png"
    rm -f "${HOME}/.local/share/pixmaps/linecounter.png"
    for sz in 512 256 128 64 48 32 24 16; do
        rm -f "${HOME}/.local/share/icons/hicolor/${sz}x${sz}/apps/com.linecounter.desktop.png"
        rm -f "${HOME}/.local/share/icons/hicolor/${sz}x${sz}/apps/linecounter.png"
    done
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "${HOME}/.local/share/applications" 2>/dev/null || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -f -t "${HOME}/.local/share/icons/hicolor" 2>/dev/null || true
    fi
    echo "✓ LineCounter has been cleanly uninstalled."
    exit 0
fi

# Target directories for unprivileged user installation
BIN_DIR="${HOME}/.local/bin"
APP_DIR="${HOME}/.local/share/applications"
ICON_DIR="${HOME}/.local/share/icons/hicolor/scalable/apps"

echo "============================================================"
echo "          Installing LineCounter Desktop Application         "
echo "============================================================"

# Ensure directories exist
mkdir -p "${BIN_DIR}"
mkdir -p "${APP_DIR}"
mkdir -p "${ICON_DIR}"

# 1. Create launcher binary in ~/.local/bin/linecounter
cat <<EOF > "${BIN_DIR}/linecounter"
#!/usr/bin/env bash
export WEBKIT_DISABLE_DMABUF_RENDERER=1
export GSK_RENDERER=ngl
if [ "\$#" -eq 0 ]; then
    exec "${SOURCE_DIR}/linecounter-desktop"
else
    exec "${SOURCE_DIR}/linecounter" "\$@"
fi
EOF
chmod +x "${BIN_DIR}/linecounter"
echo "✓ Installed executable: ${BIN_DIR}/linecounter"

# 2. Install Application Icons
mkdir -p "${HOME}/.local/share/pixmaps"
if [ -f "${SOURCE_DIR}/linecounter.svg" ]; then
    cp "${SOURCE_DIR}/linecounter.svg" "${ICON_DIR}/com.linecounter.desktop.svg"
    cp "${SOURCE_DIR}/linecounter.svg" "${ICON_DIR}/linecounter.svg"
    cp "${SOURCE_DIR}/linecounter.svg" "${HOME}/.local/share/pixmaps/com.linecounter.desktop.svg"
    cp "${SOURCE_DIR}/linecounter.svg" "${HOME}/.local/share/pixmaps/linecounter.svg"
    echo "✓ Installed scalable vector icon"
fi

if [ -f "${SOURCE_DIR}/linecounter.png" ]; then
    if python3 -c "from PIL import Image" >/dev/null 2>&1; then
        python3 -c "
import os
from PIL import Image

src_img = Image.open('${SOURCE_DIR}/linecounter.png')
home = os.environ['HOME']

for sz in [512, 256, 128, 64, 48, 32, 24, 16]:
    dir_path = f'{home}/.local/share/icons/hicolor/{sz}x{sz}/apps'
    os.makedirs(dir_path, exist_ok=True)
    resized = src_img.resize((sz, sz), Image.Resampling.LANCZOS)
    resized.save(f'{dir_path}/com.linecounter.desktop.png')
    resized.save(f'{dir_path}/linecounter.png')
"
    else
        for size in 512 256 128 64 48 32 24 16; do
            target_dir="${HOME}/.local/share/icons/hicolor/${size}x${size}/apps"
            mkdir -p "${target_dir}"
            cp "${SOURCE_DIR}/linecounter.png" "${target_dir}/com.linecounter.desktop.png"
            cp "${SOURCE_DIR}/linecounter.png" "${target_dir}/linecounter.png"
        done
    fi
    cp "${SOURCE_DIR}/linecounter.png" "${HOME}/.local/share/pixmaps/com.linecounter.desktop.png"
    cp "${SOURCE_DIR}/linecounter.png" "${HOME}/.local/share/pixmaps/linecounter.png"
fi
echo "✓ Installed application icons"

# 3. Install .desktop file (single clean entry to prevent duplicates)
rm -f "${APP_DIR}/LineCounter.desktop"
cat <<EOF > "${APP_DIR}/com.linecounter.desktop.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=LineCounter
GenericName=Code & Line Counter
Comment=Accurately count all lines in files and directories (including empty lines)
Exec=${BIN_DIR}/linecounter %F
Icon=com.linecounter.desktop
Terminal=false
Categories=Utility;Development;TextEditor;
MimeType=text/plain;text/x-python;text/x-c;text/x-c++;text/x-java;text/x-javascript;text/markdown;inode/directory;
Keywords=count;lines;code;metrics;sloc;cloc;counter;empty;whitespace;
StartupNotify=true
StartupWMClass=com.linecounter.desktop
Actions=ScanFolder;NewPaste;

[Desktop Action ScanFolder]
Name=Batch Scan Folder
Exec=${BIN_DIR}/linecounter %F

[Desktop Action NewPaste]
Name=Open Live Counter
Exec=${BIN_DIR}/linecounter
EOF
chmod +x "${APP_DIR}/com.linecounter.desktop.desktop"
echo "✓ Installed desktop entry: ${APP_DIR}/com.linecounter.desktop.desktop"

# Desktop shortcut
if [ -d "${HOME}/Desktop" ]; then
    cp "${APP_DIR}/com.linecounter.desktop.desktop" "${HOME}/Desktop/LineCounter.desktop"
    chmod +x "${HOME}/Desktop/LineCounter.desktop"
    if command -v gio >/dev/null 2>&1; then
        gio set "${HOME}/Desktop/LineCounter.desktop" metadata::trusted true 2>/dev/null || true
    fi
    echo "✓ Updated Desktop shortcut: ${HOME}/Desktop/LineCounter.desktop"
fi

# 4. Refresh Desktop & Icon Caches
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "${APP_DIR}" 2>/dev/null || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "${HOME}/.local/share/icons/hicolor" 2>/dev/null || true
fi

echo "============================================================"
echo "Installation complete!"
echo "LineCounter is now available in your Application Menu."
echo "You can also launch it from terminal using: linecounter"
echo "============================================================"

# Check PATH
case ":$PATH:" in
    *:"${BIN_DIR}":*) ;;
    *) echo "Note: Add '${BIN_DIR}' to your PATH in ~/.bashrc or ~/.zshrc if 'linecounter' is not found in terminal." ;;
esac
