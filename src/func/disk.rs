use std::{fmt::Write, path::PathBuf};

pub fn disk(
    mut shell: String,
    blk_b: String,
    blk_m: String
    ) {
    shell.write_str(
            format!(
                r#"
                mkfs.fat -F32 /dev/{blk_b}
                mkfs.ext4 /dev/{blk_m}
                "#
            )
            .as_str(),
        )
        .unwrap();

    if PathBuf::from("/sys/firmware/efi/").exists() {
        shell.write_str(
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
        shell.write_str(
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
}
