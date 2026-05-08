#!/bin/bash
# ==============================================================================
# VAJRA AUTONOMOUS CYBER SECURITY FRAMEWORK - CLOUD INSTALLER
# Created by: Chandan Pandey
# ==============================================================================

set -e

GREEN="\e[32m"
RED="\e[31m"
CYAN="\e[36m"
RESET="\e[0m"

echo -e "${RED}"
cat << "EOF"
  ██╗    ██╗    ██╗   ██╗ █████╗       ██╗██████╗  █████╗ 
████████████╗   ██║   ██║██╔══██╗      ██║██╔══██╗██╔══██╗
██╔██████╔██║   ██║   ██║███████║      ██║██████╔╝███████║
████████████║   ╚██╗ ██╔╝██╔══██║ ██   ██║██╔══██╗██╔══██║
  ██╗    ██╗     ╚████╔╝ ██║  ██║ ╚█████╔╝██║  ██║██║  ██║
██╔═╝    ╚═██╗    ╚═══╝  ╚═╝  ╚═╝  ╚════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
EOF
echo -e "${RESET}"
echo -e "${CYAN}The Ultimate Cyber Security Framework (2026 Edition)${RESET}"
echo -e "${GREEN}Author: Chandan Pandey${RESET}\n"

if [ "$1" == "uninstall" ]; then
    echo -e "${RED}[*] Uninstalling VAJRA from your system...${RESET}"
    sudo rm -rf /opt/vajra
    sudo rm -f /usr/local/bin/vajra
    echo -e "${GREEN}[+] VAJRA uninstalled successfully.${RESET}"
    exit 0
fi

echo -e "${CYAN}[*] Checking system requirements...${RESET}"

# Smart check for Rust to prevent infinite reinstalls in non-interactive shells
if ! command -v cargo &> /dev/null && [ ! -f "$HOME/.cargo/bin/cargo" ]; then
    echo -e "${RED}[!] Rust not found. Installing Rust Toolchain...${RESET}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
else
    echo -e "${GREEN}[+] Rust is already installed. Skipping...${RESET}"
fi

# Ensure cargo is in PATH for this script session
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo -e "${CYAN}[*] Checking required Linux build dependencies (OpenSSL & Pkg-Config)...${RESET}"
if command -v apt-get &> /dev/null; then
    if ! dpkg -s pkg-config libssl-dev build-essential &> /dev/null; then
        sudo apt-get update -y
        sudo apt-get install -y pkg-config libssl-dev build-essential
    else
        echo -e "${GREEN}[+] Debian/Kali dependencies already installed. Skipping...${RESET}"
    fi
elif command -v dnf &> /dev/null; then
    if ! rpm -q pkgconf-pkg-config openssl-devel &> /dev/null; then
        sudo dnf install -y pkgconf-pkg-config openssl-devel
    fi
elif command -v pacman &> /dev/null; then
    if ! pacman -Qs pkgconf openssl &> /dev/null; then
        sudo pacman -Sy --noconfirm pkgconf openssl
    fi
fi

echo -e "${CYAN}[*] Cloning VAJRA Terminal...${RESET}"
rm -rf /tmp/vajra-terminal
git clone https://github.com/thecnical/vajra-terminal.git /tmp/vajra-terminal

echo -e "${CYAN}[*] Compiling Zero-Latency Rust Matrix UI...${RESET}"
cd /tmp/vajra-terminal
cargo build --release

echo -e "${CYAN}[*] Installing VAJRA globally...${RESET}"
sudo mkdir -p /opt/vajra
sudo cp target/release/vajra-terminal /opt/vajra/vajra
sudo cp config.toml /opt/vajra/

# Creating executable wrapper
sudo bash -c 'cat << "EOF" > /usr/local/bin/vajra
#!/bin/bash
cd /opt/vajra
./vajra "$@"
EOF'

sudo chmod +x /usr/local/bin/vajra

echo -e "${GREEN}[+] Installation Complete!${RESET}"
echo -e "${CYAN}Type 'vajra' from anywhere in your Linux terminal to enter the Matrix.${RESET}"
