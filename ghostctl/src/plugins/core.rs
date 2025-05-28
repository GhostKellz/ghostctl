pub fn run_core_script(name: &str) {
    match name {
        "cleanup-logs" => {
            println!("Running cleanup logs...");
            // builtin logic or shell out
        }
        _ => println!("Unknown core plugin"),
    }
}
