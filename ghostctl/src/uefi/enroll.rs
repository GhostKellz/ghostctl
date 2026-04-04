// Secure Boot key enrollment for OVMF VARS files
//
// Uses virt-fw-vars to generate OVMF VARS files with Microsoft Secure Boot
// keys pre-enrolled, enabling Windows 11 VMs to boot with Secure Boot enabled.

use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;

/// Enroll Secure Boot keys into a new OVMF VARS file
///
/// Uses the Red Hat enrollment profile which includes Microsoft keys,
/// suitable for booting Windows 11 and signed Linux distributions.
pub fn enroll_keys(output: PathBuf, ovmf_vars_template: PathBuf, verbose: bool) -> Result<()> {
    // Validate input file exists
    if !ovmf_vars_template.exists() {
        return Err(anyhow!(
            "OVMF VARS template not found: {}\nInstall with: sudo pacman -S edk2-ovmf",
            ovmf_vars_template.display()
        ));
    }

    // Check if output already exists
    if output.exists() {
        return Err(anyhow!(
            "Output file already exists: {}\nRemove it first or choose a different path",
            output.display()
        ));
    }

    // Check virt-fw-vars is available
    if which::which("virt-fw-vars").is_err() {
        return Err(anyhow!(
            "virt-fw-vars not found\nInstall with: sudo pacman -S virt-firmware"
        ));
    }

    if verbose {
        eprintln!(
            "[ghostctl] Enrolling Secure Boot keys into {}",
            output.display()
        );
    }

    // Run virt-fw-vars to enroll keys using Red Hat profile
    // This enrolls: PK (Red Hat), KEK (Red Hat + Microsoft), db (Microsoft Windows + UEFI)
    let mut cmd = Command::new("virt-fw-vars");
    cmd.arg("-i")
        .arg(ovmf_vars_template.to_str().unwrap())
        .arg("--enroll-redhat")
        .arg("--sb")
        .arg("-o")
        .arg(output.to_str().unwrap());

    if verbose {
        cmd.arg("-v");
        eprintln!("[ghostctl] Running: {:?}", cmd);
    }

    let output_result = cmd.output().context("Failed to run virt-fw-vars")?;

    if verbose {
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        if !stdout.is_empty() {
            eprintln!("{}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("{}", stderr);
        }
    }

    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(anyhow!("virt-fw-vars failed: {}", stderr));
    }

    println!("Created OVMF VARS with Secure Boot keys enrolled");
    println!("  Output: {}", output.display());
    println!();
    println!("Usage in libvirt XML:");
    println!(
        "  <loader readonly='yes' secure='yes' type='pflash'>/usr/share/edk2/x64/OVMF_CODE.secboot.4m.fd</loader>"
    );
    println!("  <nvram>{}</nvram>", output.display());

    Ok(())
}

/// Verify that a VARS file has Secure Boot properly configured
///
/// Note: This is a best-effort check that parses virt-fw-vars output.
/// For definitive verification, boot a VM and check Secure Boot status.
pub fn verify_vars(vars_file: PathBuf, verbose: bool) -> Result<()> {
    if !vars_file.exists() {
        return Err(anyhow!("VARS file not found: {}", vars_file.display()));
    }

    // Use virt-fw-vars to dump the variables
    let output = Command::new("virt-fw-vars")
        .args(["-i", vars_file.to_str().unwrap(), "-p", "-v"])
        .output()
        .context("Failed to run virt-fw-vars")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("virt-fw-vars failed: {}", stderr));
    }

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    if verbose {
        println!("{}", combined);
    }

    // Check for key indicators in verbose output
    let has_pk = combined.contains("name=PK ");
    let has_kek = combined.contains("name=KEK ");
    let has_db = combined.contains("name=db ");
    let secure_boot = combined.contains("SecureBootEnable") && combined.contains("bool: ON");

    // Check for Microsoft certs
    let has_ms_windows = combined.contains("Microsoft Windows Production");
    let has_ms_uefi = combined.contains("Microsoft Corporation UEFI CA");

    println!("Secure Boot Status: {}", vars_file.display());
    println!();
    println!("  {} Platform Key (PK)", if has_pk { "+" } else { "-" });
    println!(
        "  {} Key Exchange Key (KEK)",
        if has_kek { "+" } else { "-" }
    );
    println!(
        "  {} Signature Database (db)",
        if has_db { "+" } else { "-" }
    );
    println!(
        "  {} Secure Boot {}",
        if secure_boot { "+" } else { "-" },
        if secure_boot { "ENABLED" } else { "DISABLED" }
    );
    println!();
    println!(
        "  {} Microsoft Windows cert",
        if has_ms_windows { "+" } else { "-" }
    );
    println!(
        "  {} Microsoft UEFI cert",
        if has_ms_uefi { "+" } else { "-" }
    );

    if has_pk && has_kek && has_db && secure_boot {
        println!();
        if has_ms_windows {
            println!("Ready for Windows 11");
        } else {
            println!("Secure Boot configured (non-Microsoft keys)");
        }
        Ok(())
    } else {
        println!();
        Err(anyhow!("Missing required Secure Boot keys"))
    }
}
