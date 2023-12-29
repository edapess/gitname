// build.rs
use std::process::Command;
use std::env;

fn main() {
    // Run the build script only in release mode
    if env::var("PROFILE").unwrap() == "release" {
        // Run the cargo build command to generate the executable
        let status = Command::new("cargo")
            .args(&["build", "--release"])
            .status()
            .expect("Failed to run cargo build command");

        // Check if the build was successful
        if !status.success() {
            panic!("Cargo build failed");
        }

        // Copy the executable to the specified location
        let source = format!("target/release/{}", env::var("gitname").unwrap());
        let destination = format!("/usr/local/bin/{}", env::var("gitname").unwrap());

        let status = Command::new("cp")
            .arg(&source)
            .arg(&destination)
            .status()
            .expect("Failed to copy executable");

        // Check if the copy was successful
        if !status.success() {
            panic!("Failed to copy executable to /usr/local/bin");
        }

        println!("Build script completed successfully");
    }
}
