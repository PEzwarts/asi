use std::{fmt::Write, process::Command};

pub fn user(
    mut shell: String,
    mut host: String,
    mut rpass: String,
    mut users: Vec<String>,
    mut upass: Vec<String>
    ) -> (String, String, String, Vec<String>, Vec<String>) {
    shell.write_str(
            format!(
                r#"
                sed -i 's/# Defaults targetpw/Defaults targetpw/g' /etc/sudoers
                sed -i 's/# ALL ALL=(ALL:ALL) ALL/ALL ALL=(ALL:ALL) ALL/g' /etc/sudoers
                "#
            )
            .as_str(),
        )
        .unwrap();

    println!("Choose an hostname:");
    host = text_io::read!();

    Command::new("clear").status().unwrap();

    println!("Choose an root passwd:");
    rpass = text_io::read!();

    Command::new("clear").status().unwrap();

    println!("Create an X amount of users:");
    let q_users: usize = text_io::read!();

    Command::new("clear").status().unwrap();

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
            shell.write_str(
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

    shell.write_str(
            format!(
                r#"
                sed -i 's/#en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/g' /etc/locale.gen
                echo 'LANG=en_US.UTF-8' > /etc/locale.conf
                echo 'KEYMAP=us' > /etc/vconsole.conf

                timedatectl
                #ln -sf /usr/share/zoneinfo/ /etc/localtime
                hwclock --systohc

                echo '{host}' > /etc/hostname
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

    return (shell, host, rpass, users, upass);
}
