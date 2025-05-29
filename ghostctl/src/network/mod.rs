pub mod dns;
pub mod mesh;
pub mod netcat;

#[allow(dead_code)]
pub fn ghostcat(host: &str, port: u16) {
    netcat::check_port(host, port);
}

#[allow(dead_code)]
pub fn dns_lookup(domain: &str) {
    use std::process::Command;
    let output = Command::new("dig").arg("+short").arg(domain).output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to run dig: {}", e),
    }
}

#[allow(dead_code)]
pub fn dnssec_check(domain: &str) {
    use std::process::Command;
    let output = Command::new("dig").arg("+dnssec").arg(domain).output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to run dig: {}", e),
    }
}

#[allow(dead_code)]
pub fn netcat(host: &str, port: u16) {
    use std::net::TcpStream;
    match TcpStream::connect((host, port)) {
        Ok(_) => println!("Port {} on {} is open", port, host),
        Err(e) => println!("Port {} on {} is closed: {}", port, host, e),
    }
}

#[allow(dead_code)]
pub fn route() {
    use std::process::Command;
    let output = Command::new("ip").arg("route").output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to run ip route: {}", e),
    }
}

#[allow(dead_code)]
pub fn dns(domain: &str) {
    dns_lookup(domain);
}

#[allow(dead_code)]
pub fn dnssec(domain: &str) {
    dnssec_check(domain);
}

#[allow(dead_code)]
pub fn gc(host: &str, port: u16) {
    println!("Ghostcat (branded netcat):");
    netcat(host, port);
}
