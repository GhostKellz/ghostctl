use std::process::Command;

#[allow(dead_code)]
pub fn lookup(domain: &str) {
    println!("Resolving DNS for {}", domain);
    let _ = Command::new("dig")
        .arg(domain)
        .status()
        .expect("Failed to execute dig");
}

#[allow(dead_code)]
pub fn check_dnssec(domain: &str) {
    println!("Checking DNSSEC for {}", domain);
    let _ = Command::new("dig")
        .args(["+dnssec", "+multi", domain])
        .status()
        .expect("Failed to execute DNSSEC dig");
}
