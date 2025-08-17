
# Version 1.0.0-s (stable)

ASI is an Rust based Open-Source Project.

ASI is an acronym which stands for Arch System Installer, intended to be used to install an base Arch Operating System on your computer. It comes with pre-installed bash scripts in the root folder to harden or install caliber class software depending on your needs, whether it is used for servers or personal use. ASI is the way to go!

# Usage with Ventoy

./build.sh

umount /dev/Ventoy

*Boot into Arch's ISO

*Use IWD/iwctl or use an Ethernet cable

./arch

*After booting into Arch system

sudo systemctl enable --now NetworkManager wpa_supplicant

cd ../..

cat ./README.md
