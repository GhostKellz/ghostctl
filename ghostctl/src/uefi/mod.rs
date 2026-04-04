// UEFI Secure Boot management for QEMU/KVM virtual machines
//
// Generates OVMF VARS files with Microsoft Secure Boot keys pre-enrolled,
// enabling Windows 11 VMs to boot with Secure Boot enabled.
//
// Uses virt-fw-vars from the virt-firmware package.

pub mod enroll;

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

/// Default path for OVMF VARS template on Arch Linux
pub const DEFAULT_OVMF_VARS: &str = "/usr/share/edk2/x64/OVMF_VARS.4m.fd";

pub fn command() -> Command {
    Command::new("uefi")
        .about("UEFI Secure Boot management for VMs")
        .subcommand_required(true)
        .subcommand(
            Command::new("enroll")
                .about("Create OVMF VARS with Secure Boot keys for Windows 11")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output path for enrolled VARS file")
                        .value_name("FILE")
                        .required(true),
                )
                .arg(
                    Arg::new("template")
                        .long("template")
                        .help("Path to OVMF_VARS.fd template")
                        .value_name("FILE")
                        .default_value(DEFAULT_OVMF_VARS),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show detailed output")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Check if VARS file has Secure Boot keys enrolled")
                .arg(
                    Arg::new("file")
                        .help("VARS file to verify")
                        .value_name("FILE")
                        .required(true),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show full variable dump")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("status").about("Check OVMF firmware and tools"))
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        Some(("enroll", sub_matches)) => {
            let output = sub_matches.get_one::<String>("output").unwrap();
            let template = sub_matches.get_one::<String>("template").unwrap();
            let verbose = sub_matches.get_flag("verbose");

            enroll::enroll_keys(PathBuf::from(output), PathBuf::from(template), verbose)
        }
        Some(("verify", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let verbose = sub_matches.get_flag("verbose");
            enroll::verify_vars(PathBuf::from(file), verbose)
        }
        Some(("status", _)) => show_status(),
        _ => unreachable!(),
    }
}

fn show_status() -> anyhow::Result<()> {
    use std::path::Path;

    println!("UEFI Secure Boot Dependencies\n");

    let ovmf_code = "/usr/share/edk2/x64/OVMF_CODE.secboot.4m.fd";
    let ovmf_vars = DEFAULT_OVMF_VARS;

    let files = [
        ("OVMF Code (Secure Boot)", ovmf_code),
        ("OVMF VARS Template", ovmf_vars),
    ];

    for (name, path) in files {
        let exists = Path::new(path).exists();
        let status = if exists { "+" } else { "-" };
        println!("{} {} - {}", status, name, path);
    }

    let has_virt_fw = which::which("virt-fw-vars").is_ok();
    let status = if has_virt_fw { "+" } else { "-" };
    println!("{} virt-fw-vars", status);

    println!();
    if !Path::new(ovmf_code).exists() || !Path::new(ovmf_vars).exists() {
        println!("Install: sudo pacman -S edk2-ovmf");
    }
    if !has_virt_fw {
        println!("Install: sudo pacman -S virt-firmware");
    }

    if Path::new(ovmf_code).exists() && Path::new(ovmf_vars).exists() && has_virt_fw {
        println!("Ready");
    }

    Ok(())
}
