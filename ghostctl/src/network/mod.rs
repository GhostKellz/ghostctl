pub mod netcat;
pub mod dns;
pub mod mesh;

pub fn ghostcat(host: &str, port: u16) {
    netcat::check_port(host, port);
}

pub fn dns_lookup(domain: &str) {
    dns::lookup(domain);
}

pub fn start_mesh() {
    mesh::tailscale_up();
}
