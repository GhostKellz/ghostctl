pub fn optimize() {
    println!("ghostctl :: NVIDIA Optimization");
    println!("- Enabling performance mode (persistence mode)...");
    let status = std::process::Command::new("nvidia-smi")
        .args(["-pm", "1"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Performance mode enabled."),
        _ => println!("Failed to enable performance mode."),
    }
    // Add more optimizations as needed
}
