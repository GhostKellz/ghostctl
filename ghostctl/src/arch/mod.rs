pub mod archfix;
pub mod perf;
pub mod pkgfix;

pub fn fix(target: String) {
    match target.as_str() {
        "pacman" => archfix::fix_pacman(),
        "pkgbuild" => pkgfix::fix_pkgbuild(),
        "optimize" => perf::tune(),
        _ => println!("Unknown arch fix target"),
    }
}
