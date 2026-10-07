#!/usr/bin/env bash
#
# VortexPlayer One-Click Installer for Linux & macOS
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/sidhantjohnaind/VortexPlayer/main/install.sh | bash
#

set -e

REPO="sidhantjohnaind/VortexPlayer"
VERSION="${1:-latest}"

BOLD='\033[1m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

printf "${CYAN}==========================================================${NC}\n"
printf "${CYAN}        🌌 VortexPlayer One-Click Unix Installer          ${NC}\n"
printf "${CYAN}==========================================================${NC}\n\n"

# 1. OS & Architecture Detection
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    OS_KEY="linux"
    ;;
  Darwin)
    OS_KEY="macos"
    ;;
  *)
    printf "${RED}Unsupported Operating System: %s${NC}\n" "$OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    ARCH_KEY="x86_64"
    ;;
  aarch64|arm64)
    ARCH_KEY="arm64"
    ;;
  armv7*|armhf)
    ARCH_KEY="armv7"
    ;;
  riscv64)
    ARCH_KEY="riscv64"
    ;;
  *)
    printf "${RED}Unsupported Architecture: %s${NC}\n" "$ARCH"
    exit 1
    ;;
esac

printf "${YELLOW}🔍 Detected System:${NC} %s (%s)\n" "$OS" "$ARCH"

# 2. Resolve Tag
TAG="$VERSION"
if [ "$VERSION" = "latest" ]; then
  printf "${YELLOW}🔍 Resolving latest release version...${NC}\n"
  if command -v curl >/dev/null 2>&1; then
    TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | head -n1 | cut -d '"' -f 4 || true)
  fi
  if [ -z "$TAG" ]; then
    TAG="v1.1.0"
  fi
fi
printf "${GREEN}✔ Selected Release:${NC} %s\n" "$TAG"

# 3. Determine Asset Name
ASSET="VortexPlayer-${OS_KEY}-${ARCH_KEY}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

printf "${YELLOW}⬇ Downloading %s...${NC}\n" "$ASSET"
if command -v curl >/dev/null 2>&1; then
  curl -fSL "$DOWNLOAD_URL" -o "${TMP_DIR}/${ASSET}"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$DOWNLOAD_URL" -O "${TMP_DIR}/${ASSET}"
else
  printf "${RED}Error: curl or wget is required to download.${NC}\n"
  exit 1
fi

printf "${YELLOW}📦 Extracting release archive...${NC}\n"
tar -xzf "${TMP_DIR}/${ASSET}" -C "$TMP_DIR"

# 4. Installation Directory
if [ "$(id -u)" -eq 0 ]; then
  INSTALL_DIR="/usr/local/bin"
else
  INSTALL_DIR="${HOME}/.local/bin"
fi
mkdir -p "$INSTALL_DIR"

BIN_SRC="$(find "$TMP_DIR" -type f -name "vortex-player*" | head -n1)"
if [ -z "$BIN_SRC" ]; then
  printf "${RED}Error: Could not locate executable in extracted archive.${NC}\n"
  exit 1
fi

chmod +x "$BIN_SRC"
cp "$BIN_SRC" "${INSTALL_DIR}/vortex-player-egui"
ln -sf "${INSTALL_DIR}/vortex-player-egui" "${INSTALL_DIR}/vortex"

printf "${GREEN}✔ Installed binary:${NC} %s/vortex-player-egui\n" "$INSTALL_DIR"
printf "${GREEN}✔ Created symlink:${NC}  %s/vortex\n" "$INSTALL_DIR"

# 5. Linux Desktop Integration (.desktop file)
if [ "$OS_KEY" = "linux" ]; then
  DESKTOP_DIR="${HOME}/.local/share/applications"
  mkdir -p "$DESKTOP_DIR"
  cat > "${DESKTOP_DIR}/vortexplayer.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=VortexPlayer
GenericName=Media Player
Comment=High-Performance Modern Media Player
Exec=${INSTALL_DIR}/vortex %U
Icon=multimedia-video-player
Terminal=false
Categories=AudioVideo;Player;Video;Audio;
MimeType=video/mp4;video/mkv;video/webm;video/avi;video/quicktime;audio/mpeg;audio/flac;audio/ogg;audio/wav;
StartupWMClass=vortex-player-egui
EOF
  chmod 644 "${DESKTOP_DIR}/vortexplayer.desktop"
  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
  fi
  printf "${GREEN}✔ Desktop launcher created:${NC} %s/vortexplayer.desktop\n" "$DESKTOP_DIR"
fi

# 6. Check PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    printf "\n${YELLOW}⚠ Note: %s is not currently in your \$PATH.${NC}\n" "$INSTALL_DIR"
    printf "  Add it by running:\n"
    printf "    ${BOLD}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}\n"
    printf "  to your ~/.bashrc or ~/.zshrc\n"
    ;;
esac

printf "\n${GREEN}🎉 Installation successful!${NC}\n"
printf "Run ${BOLD}vortex${NC} or launch VortexPlayer from your application menu.\n\n"
