fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Copy VERSION file. Do not fail e.g. when built via `cargo publish`
    let _ = std::fs::copy("../../VERSION", "./src/VERSION");
    Ok(())
}
