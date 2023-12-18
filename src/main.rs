use builder::build_canary;
use html::build_html;
use signer::sign_canary;
use std::env;

mod builder;
mod html;
mod scraper;
mod signer;

/// Struct to hold canary arguments
pub struct CanaryConfig {
    pub(crate) domain_name: String,
    pub(crate) expiration_timer: u64,
    pub(crate) gpg_key_id: String,
    pub(crate) output_dir: String,
}
/// Main function
pub fn main() {
    // Collect canary arguments from command line
    let args: Vec<String> = env::args().collect();

    // Map canary arguments to CanaryConfig struct
    let configuration = CanaryConfig {
        domain_name: args[1].clone(),
        expiration_timer: args[2].clone().parse().unwrap(),
        gpg_key_id: args[3].clone(),
        output_dir: args[4].clone(),
    };

    // Specify path to canary txt file
    let canary_path = "./canary.txt";

    // Start building canary
    build_canary(&configuration, &canary_path).expect("Error building canary.txt");

    // Sign the canary_elements
    println!("Preparing to sign canary...");
    sign_canary(&configuration, &canary_path).expect("Error signing canary.txt");

    // Check if canary.txt.asc exists
    let signed_path = format!("{}.asc", &canary_path);
    if !std::path::Path::new(&signed_path).exists() {
        println!("Error signing canary.txt, check that singing key and password are correct.");
        std::process::exit(1);
    }
    println!(
        "Canary signed successfully by: {} ./canary.txt.asc",
        configuration.gpg_key_id
    );

    // Build html file
    build_html(&configuration, &canary_path).expect("Error building canary HTML file");
    println!(
        "Canary HTML file generated successfully: {}",
        configuration.output_dir
    );
}
