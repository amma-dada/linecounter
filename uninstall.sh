#!/usr/bin/env bash
# LineCounter Uninstaller

rm -f "${HOME}/.local/bin/linecounter"
rm -f "${HOME}/.local/share/applications/LineCounter.desktop"
rm -f "${HOME}/.local/share/icons/hicolor/scalable/apps/linecounter.svg"
rm -f "${HOME}/.local/share/pixmaps/linecounter.svg"
rm -f "${HOME}/.local/share/pixmaps/linecounter.png"

for size in 256 128 64 48 32 16; do
    rm -f "${HOME}/.local/share/icons/hicolor/${size}x${size}/apps/linecounter.png"
done

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "${HOME}/.local/share/applications" 2>/dev/null || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "${HOME}/.local/share/icons/hicolor" 2>/dev/null || true
fi

echo "LineCounter has been cleanly uninstalled from your user environment."
