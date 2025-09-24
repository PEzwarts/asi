use std::{
    fmt::Write,
    fs::{self, File},
    path::PathBuf,
    process::{exit, Command},
};

fn main() {
    let mut shell: String = String::new();

    let mut rpass: String = String::new();
    let mut upass: Vec<String> = Vec::new();
    let mut users: Vec<String> = Vec::new();

    let mut cmp: String = String::new();
    let mut de: String = String::new();
    let mut gr: String = String::new();

    let mut blk: String = String::new();
    let mut blk_b: String = String::new();
    let mut blk_s: String = String::new();
    let mut blk_m: String = String::new();

    let mut sec_krl: String = String::from(
        r#"
# https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html
# https://tails.net/contribute/design/kernel_hardening/
# https://github.com/torvalds/linux/blob/master/Documentation/security/self-protection.rst

# Ptrace restriction.
sysctl -w kernel.yama.ptrace_scope=2

# Improve ASLR randomization.
vm.mmap_rnd_bits=32
vm.mmap_rnd_compat_bits=16
kernel.randomize_va_space=2

# Prevents kexec from replacing the running kernel.
kernel.kexec_load_disabled=1

# Prevents malicious software from reading exposed kernel addresses from /proc/kallsyms .
kernel.kptr_restrict=2

# Randomizes kernel stack offset on syscall entry.
sysctl -w randomize_kstack_offset=on

# Prevents kernel addresses exposure via dmesg.
sysctl -w kernel.dmesg_restrict=1

# Disable modules loading.
sysctl -w kernel.modules_disabled=1

# Disable user namespaces.
sysctl -w user.max_user_namespaces=0
        "#,
    );

    let mut sec_net: String = String::from(
        r#"
# https://tails.net/contribute/design/kernel_hardening/
# https://kspp.github.io/Recommended_Settings

# Prevent JIT-spraying.
net.core.bpf_jit_harden = 2
        "#,
    );

    let mut sec_fs: String = String::from(
        r#"
# https://kspp.github.io/Recommended_Settings

sysctl -w fs.protected_regular=2
sysctl -w fs.protected_fifos=2

sysctl -w fs.protected_hardlinks=1
sysctl -w fs.protected_symlinks=1
        "#,
    );

    // Mirrorlist repos

    let chaotic_cx: String = String::from(
        r#"
if [ ! -f /etc/pacman.d/chaotic-mirrorlist ]; then
    sudo pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
    sudo pacman-key --lsign-key 3056513887B78AEB

    sudo pacman --noconfirm -U https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst
    sudo pacman --noconfirm -U https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst
    echo '

[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist
    ' >> /etc/pacman.conf
fi
        "#,
    );

    let black_arch: String = String::from(
        r#"
if [ ! -f /etc/pacman.d/blackarch-mirrorlist]; then
    curl -O https://blackarch.org/strap.sh

    # SHA1 hash might change !

    echo bbf0a0b838aed0ec05fff2d375dd17591cbdf8aa strap.sh | sha1sum -c
    chmod u+x ./strap.sh
    sudo ./strap.sh
fi
        "#,
    );

    // Shell scripts
    let min: String = String::from(
        format!(
            r#"
echo ## MINIMAL SCRIPT ##

echo ## Enabling NetworkManager ##
echo ## Reboot needed! Execute min.sh again after reboot! ##
echo ## Type CTRL-C to abort ##

sudo systemctl enable --now NetworkManager wpa_supplicant

sleep 3

echo [REBOOTING] .
sleep 1
echo [REBOOTING] ..
sleep 1
echo [REBOOTING] ...
sleep 1

reboot

# Basic caliber class software.
sudo pacman -Syu --noconfirm librewolf torbrowser-launcher vlc qbittorrent

# Cryptographic utils.
sudo pacman -S --noconfirm gnupg

# Filesystem formats.
sudo pacman -S --noconfirm ntfs-3g exfat-utils udiskie

{chaotic_cx}
            "#,
        )
        .as_str(),
    );

    let srv: String = String::from(
        format!(
            r#"
echo ## SERVER SCRIPT ##

echo ## Enabling NetworkManager ##
echo ## Reboot needed! Execute min.sh again after reboot! ##
echo ## Type CTRL-C to abort ##

sudo systemctl enable --now NetworkManager wpa_supplicant

sleep 3

echo [REBOOTING] .
sleep 1
echo [REBOOTING] ..
sleep 1
echo [REBOOTING] ...
sleep 1

{chaotic_cx}

# Network utils.
sudo pacman -Syu --noconfirm ufw

# Cryptograhpic utils.
sudo pacman -S --noconfirm gnupg

# Filesystem formats.
sudo pacman -S --noconfirm ntfs-3g exfat-utils udiskie
            "#,
        )
        .as_str(),
    );

    Command::new("lsblk").status().unwrap();
    println!("Choose your drive /dev/... :");
    blk = text_io::read!();

    Command::new("wipefs")
        .args(["--all", format!("/dev/{blk}").as_str()])
        .status()
        .unwrap();

    println!("NOTE: boot partition must partitioned with at least an minimum of 1GB!\n1; cfdisk 2; fdisk:");
    let q_boot: usize = text_io::read!();

    // Partition

    match q_boot {
        1 => {
            Command::new("cfdisk")
                .arg(format!("/dev/{blk}"))
                .status()
                .unwrap();
        }
        2 => {
            Command::new("fdisk")
                .arg(format!("/dev/{blk}"))
                .status()
                .unwrap();
        }
        _ => exit(0),
    }

    println!("Choose boot partition /dev/... :");
    blk_b = text_io::read!();

    println!("Choose base partition /dev/... :");
    blk_m = text_io::read!();

    // Formatting

    println!("Type Return/Enter key for no swap\nChoose an swap partition /dev/... :");
    blk_s = text_io::read!();
    if blk_s.len() > 0 {
        shell
            .write_str(
                format!(
                    "
                    mkswap /dev/{blk_s}
                    swapon /dev/{blk_s}
                    "
                )
                .as_str(),
            )
            .unwrap();
    }

    println!("Choose an hostname:");
    cmp = text_io::read!();

    println!("Choose an root passwd:");
    rpass = text_io::read!();

    println!("[gnome, plasma-desktop, xfce4, ..., none]\nChoose an desktop enviroment:");
    de = text_io::read!();

    println!("[gdm, sddm, lightdm, ..., none]\nChoose an greeter:");
    gr = text_io::read!();

    shell
        .write_str(
            format!(
                r#"
                mkfs.fat -F32 /dev/{blk_b}
                mkfs.ext4 /dev/{blk_m}
                "#
            )
            .as_str(),
        )
        .unwrap();

    // Mount

    if PathBuf::from("/sys/firmware/efi/").exists() {
        shell
            .write_str(
                format!(
                    r#"
                    mount /dev/{blk_m} /mnt
                    mount --mkdir /dev/{blk_b} /mnt/boot/efi
                    "#
                )
                .as_str(),
            )
            .unwrap();
    } else {
        shell
            .write_str(
                format!(
                    r#"
                    mount /dev/{blk_m} /mnt
                    mount --mkdir /dev/{blk_b} /mnt/boot
                    "#
                )
                .as_str(),
            )
            .unwrap();
    }

    // Refresh keys

    shell
        .write_str(
            format!(
                r#"
                pacman-key --init
                pacman-key --populate
                "#
            )
            .as_str(),
        )
        .unwrap();

    // Install

    if de.to_lowercase().contains("none") || gr.to_lowercase().contains("none") {
        shell
            .write_str(
                format!(
                    r#"
                    pacstrap -K /mnt base base-devel linux-lts linux-lts-headers linux-firmware grub efibootmgr xorg wayland amd-ucode intel-ucode alsa-firmware alsa-utils pulseaudio networkmanager wpa_supplicant reflector kitty git wget vim
                    genfstab -U /mnt >> /mnt/etc/fstab
                    arch-chroot /mnt /bin/bash -c "
                    "#
                )
                .as_str(),
            )
            .unwrap();
    } else {
        shell
            .write_str(
                format!(
                    r#"
                    pacstrap -K /mnt base base-devel linux-lts linux-lts-headers linux-firmware grub efibootmgr xorg wayland amd-ucode intel-ucode alsa-firmware alsa-utils pulseaudio networkmanager wpa_supplicant reflector {de} {gr} kitty git wget vim
                    genfstab -U /mnt >> /mnt/etc/fstab
                    arch-chroot /mnt /bin/bash -c "
                    "#
                )
                .as_str(),
            )
            .unwrap();
    }

    // Security: Lockout

    shell
        .write_str(
            format!(
                r#"
                #echo 'auth optional pam_faildelay.so delay=8000000' > /etc/pam.d/system-login
                #sed -i 's/# deny = 3/deny = 2/g' /etc/security/faillock.conf
                #sed -i 's/# fail_interval = 900/fail_interval = 200000000/g' /etc/security/faillock.conf
                #sed -i 's/# unlock_time = 600/unlock_time = 6000000000/g' /etc/security/faillock.conf
                "#
            )
            .as_str(),
        )
        .unwrap();

    // Users

    shell
        .write_str(
            format!(
                r#"
                sed -i 's/# Defaults targetpw/Defaults targetpw/g' /etc/sudoers
                sed -i 's/# ALL ALL=(ALL:ALL) ALL/ALL ALL=(ALL:ALL) ALL/g' /etc/sudoers
                "#
            )
            .as_str(),
        )
        .unwrap();

    println!("Create an X amount of users:");
    let q_users: usize = text_io::read!();

    for i in 0..q_users {
        println!("Choose an username for user{}", i + 1);
        let tmp_user = text_io::read!();
        println!("Choose an userpass for user{}", i + 1);
        let tmp_pass = text_io::read!();
        users.push(tmp_user);
        upass.push(tmp_pass);
    }

    for user in &users {
        for pass in &upass {
            shell
                .write_str(
                    format!(
                        r#"
                        useradd -m -G users -s /bin/bash {user}
                        (echo '{pass}'; echo '{pass}') | passwd {user}
                        "#
                    )
                    .as_str(),
                )
                .unwrap();
        }
    }

    // Config

    shell
        .write_str(
            format!(
                r#"
                sed -i 's/#en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/g' /etc/locale.gen
                echo 'LANG=en_US.UTF-8' > /etc/locale.conf
                echo 'KEYMAP=us' > /etc/vconsole.conf

                timedatectl
                #ln -sf /usr/share/zoneinfo/ /etc/localtime
                hwclock --systohc

                echo '{cmp}' > /etc/hostname
                (echo '{rpass}'; echo '{rpass}') | passwd root

                sudo locale-gen

                reflector --save /etc/pacman.d/mirrorlist

                echo '

[multilib]
Include = /etc/pacman.d/mirrorlist
                ' >> /etc/pacman.conf
                "#
            )
            .as_str(),
        )
        .unwrap();

    // Bootloader

    if PathBuf::from("/sys/firmware/efi/").exists() {
        shell
            .write_str(
                format!(
                    r#"
                    grub-install /dev/{blk} --target=x86_64-efi --efi-directory=/boot/efi/
                    grub-mkconfig -o /boot/grub/grub.cfg
                    mkinitcpio -P
                    "#
                )
                .as_str(),
            )
            .unwrap();
    } else {
        shell
            .write_str(
                format!(
                    r#"
                    grub-install /dev/{blk}
                    grub-mkconfig -o /boot/grub/grub.cfg
                    mkinitcpio -P
                    "#
                )
                .as_str(),
            )
            .unwrap();
    }

    // Services

    shell
        .write_str(
            format!(
                r#"
                systemctl enable {gr}
                "
                "#
            )
            .as_str(),
        )
        .unwrap();

    File::create("./arch.sh").unwrap();
    fs::write("./arch.sh", shell.as_str()).unwrap();
    Command::new("/bin/bash").arg("./arch.sh").status().unwrap();

    File::create("/mnt/README.md").unwrap();
    fs::write(
        "/mnt/README.md",
        format!(
            "
min.sh is used for an minimal installation of software used by many.

srv.sh is used for an minimal installation of software used by server maintainers.

harden.sh is used to harden your current system to be more resistant towards malware. This is optional.
            "
        )
        .as_str(),
    )
    .unwrap();

    File::create("/mnt/min.sh").unwrap();
    Command::new("chmod")
        .args(["u+x", "/mnt/min.sh"])
        .status()
        .unwrap();
    fs::write("/mnt/min.sh", min).unwrap();

    File::create("/mnt/srv.sh").unwrap();
    Command::new("chmod")
        .args(["u+x", "/mnt/srv.sh"])
        .status()
        .unwrap();
    fs::write("/mnt/srv.sh", srv).unwrap();

    File::create("/mnt/harden.sh").unwrap();
    Command::new("chmod")
        .args(["u+x", "/mnt/harden.sh"])
        .status()
        .unwrap();
    fs::write("/mnt/harden.sh", format!("{sec_krl}{sec_net}{sec_fs}")).unwrap();

    Command::new("umount")
        .args(["-r", "/mnt"])
        .status()
        .unwrap();

    Command::new("umount").arg("-a").status().unwrap();
    Command::new("shutdown").arg("now").status().unwrap();
}
