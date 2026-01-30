use std::fmt::Write;

pub fn base(mut shell: String, mut de: String, mut gr: String) -> (String, String, String) {
    println!("[gnome, plasma-desktop, xfce4, ..., none]\nChoose an desktop enviroment:");
    de = text_io::read!();

    println!("[gdm, sddm, lightdm, ..., none]\nChoose an greeter:");
    gr = text_io::read!();

    shell.write_str(
            format!(
                r#"
                pacman-key --init
                pacman-key --populate
                "#
            )
            .as_str(),
        )
        .unwrap();

    if de.to_lowercase().contains("none") || gr.to_lowercase().contains("none") {
        shell.write_str(
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

        return (shell, de, gr);
    } else {
        shell.write_str(
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

        return (shell, de, gr);
    }
}
