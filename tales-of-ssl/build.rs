use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=.env");

    let env_path = Path::new(".env");
    if !env_path.exists() {
        println!("cargo::error={} file not found.", env_path.display());
    }
}
