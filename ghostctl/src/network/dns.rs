use std::process::Command;

pub fn lookup(domain: &str) {
    println!("Resolving DNS for {}", domain);
    let _ = Command::new("dig")
        .arg(domain)
        .status()
        .expect("Failed to execute dig");
}

pub fn check_dnssec(domain: &str) {
    println!("Checking DNSSEC for {}", domain);
    let _ = Command::new("dig")
        .args(["+dnssec", "+multi", domain])
        .status()
        .expect("Failed to execute DNSSEC dig");
}
