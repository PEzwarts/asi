use std::{fmt::Write, path::PathBuf};

pub fn boot(mut shell: String, blk: String) -> String {
    if PathBuf::from("/sys/firmware/efi/").exists() {
        shell.write_str(
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

        return shell;
    } else {
        shell.write_str(
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

        return shell;
    }
}
