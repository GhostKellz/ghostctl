pub mod external;

pub use external::*;

// Re-export from external module
pub use external::install_acme_sh;
pub use external::list_certificates;
pub use external::renew_certificates;

// Add missing function stubs
pub fn issue_certificate_for_domain(domain: &str) {
    println!("📜 Issuing certificate for domain: {}", domain);
    external::issue_certificate();
}
