mod func;

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

    let mut host: String = String::new();
    let mut de: String = String::new();
    let mut gr: String = String::new();

    let mut blk: String = String::new();
    let mut blk_b: String = String::new();
    let mut blk_s: String = String::new();
    let mut blk_m: String = String::new();

    let mut sec_krl: String = format!(include_str!("./bash/sec_krl.sh")).to_string();
    let mut sec_net: String = format!(include_str!("./bash/sec_net.sh")).to_string();
    let mut sec_fs: String = format!(include_str!("./bash/sec_fs.sh")).to_string();

    // Mirrorlist repos

    let chaotic_cx: String = format!(include_str!("./bash/chaotic_cx.sh")).to_string();
    let black_arch: String = format!(include_str!("./bash/black_arch.sh")).to_string();

    // Shell scripts

    let min: String = format_args!(include_str!("./bash/min.sh"), chaotic_cx).to_string();
    let srv: String = format_args!(include_str!("./bash/srv.sh"), chaotic_cx).to_string();

    // Partition

    let (
        mut shell,
        mut blk,
        mut blk_b,
        mut blk_s,
        mut blk_m
        ) = func::part::part(shell, blk, blk_b, blk_s, blk_m);

    let (
        mut shell,
        mut blk_b,
        mut blk_m
        ) = func::disk::disk(shell.clone(), blk_b, blk_m);

    let (
        mut shell,
        mut de,
        mut gr,
        ) = func::base::base(shell, de, gr);

    let (
        mut shell,
        mut host,
        mut rpass,
        mut users,
        mut upass
        ) = func::user::user(shell, host, rpass, users, upass);

    let (
        mut shell,
        mut blk
        ) = func::boot::boot(shell, blk);

    shell.write_str(
            format!(
                r#"
                systemctl enable {gr}
                "
                "#
            )
            .as_str(),
        )
        .unwrap();

    File::create("./asi.sh").unwrap();
    fs::write("./asi.sh", shell.as_str()).unwrap();
    Command::new("/bin/bash").arg("./asi.sh").status().unwrap();

    File::create("/mnt/README.md").unwrap();
    fs::write(
        "/mnt/README.md",
        format!(
"min.sh is used for an minimal installation of software used by many.
srv.sh is used for an minimal installation of software used by system engineers.
harden.sh is used to harden your current system to be more resistant towards malware. This is optional."
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
