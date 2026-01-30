echo ## MINIMAL SCRIPT ##

echo ## Enabling NetworkManager ##
echo ## Reboot needed! Execute min.sh again after reboot! ##
echo ## Type CTRL-C to abort ##

sudo systemctl enable --now NetworkManager wpa_supplicant

{}

echo [INSTALL] .
sleep 1
echo [INSTALL] ..
sleep 1
echo [INSTALL] ...
sleep 1

# Basic caliber class software
sudo pacman -Syu --noconfirm librewolf torbrowser-launcher vlc qbittorrent

# Cryptographic utils
sudo pacman -S --noconfirm gnupg

# Filesystem formats
sudo pacman -S --noconfirm ntfs-3g exfat-utils udiskie
