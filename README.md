
## Version 1.2.2-s (stable)

ASI (Arch System Installer) is an Rust based Open-Source project.

This project is intended to be used to install an base Arch Operating System on your computer. It comes with pre-installed bash scripts in the root directory to harden or install software depending on your needs, whether it is used for servers or personal use. ASI is the way to go!

### Features

* Minimal
* ChaoticCX
* Security; ~/../../harden.sh

### Usage with Ventoy

Run these following bash commands with its associated ISO enviroment.

./build.sh
umount /path/to/ventoy

Use an Arch ISO to boot into an post-install enviroment.

mkdir mnt
mount /path/to/ventoy ./mnt
cd ./mnt

Connect to wifi before running the ASI binary.

./arch

After booting into your system.

sudo systemctl enable --now NetworkManager wpa_supplicant

cd ../..

cat ./README.md
