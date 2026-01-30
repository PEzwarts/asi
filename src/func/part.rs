use std::{fmt::Write, process::Command};

pub fn part(
    mut shell: String,
    mut blk: String,
    mut blk_b: String,
    mut blk_s: String,
    mut blk_m: String
    ) -> (String, String, String, String, String) {
    Command::new("lsblk").status().unwrap();

    println!("Choose your drive /dev/... :");
    blk = text_io::read!();

    Command::new("clear").status().unwrap();

    Command::new("wipefs")
        .args(["--all", format!("/dev/{blk}").as_str()])
        .status()
        .unwrap();

    println!("NOTE: boot partition must partitioned with at least an minimum of 1GB!\n1; cfdisk 2; fdisk:");
    let q_boot: usize = text_io::read!();

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
        _ => print!("")
    }

    Command::new("clear").status().unwrap();

    println!("Choose boot partition /dev/... :");
    blk_b = text_io::read!();

    println!("Choose base partition /dev/... :");
    blk_m = text_io::read!();

    println!("Type Return/Enter key for no swap\nChoose an swap partition /dev/... :");
    blk_s = text_io::read!();

    if blk_s.len() > 0 {
        shell.write_str(
            format!(
                "
                mkswap /dev/{blk_s}
                swapon /dev/{blk_s}
                "
                )
                .as_str()
            )
            .unwrap();
    }

    return (
        shell,
        blk,
        blk_b,
        blk_s,
        blk_m);
}
