// This module generates a GPG signature of the canary.txt file
use crate::CanaryConfig;
use std::io;
use std::process::Command;

// Function to sign the canary_elements.txt file
pub fn sign_canary(configuration: &CanaryConfig, canary_path: &str) -> io::Result<()> {
    let output = Command::new("gpg")
        .args([
            "--clear-sign",
            "--armor",
            "--local-user",
            &configuration.gpg_key_id,
            "--pinentry-mode",
            "loopback",
            canary_path,
        ])
        .output()
        .expect("failed to sign canary.txt");
    // Print any output from the command
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
// gpg --clear-sign --default-key canary --pinentry-mode loopback --passphrase password canary.txt
