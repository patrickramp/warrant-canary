use builder::build_canary;
use html::build_html;
use signer::sign_canary;
use std::env;

mod builder;
mod html;
mod scraper;
mod signer;

// Struct to hold canary arguments
pub struct CanaryConfig<'a> {
    pub(crate) domain_name: &'a String,
    pub(crate) expiration_timer: &'a u64,
    pub(crate) gpg_key_id: &'a String,
    pub(crate) output_dir: &'a String,
}
// Main function
fn main() {
    // Collect canary arguments from command line
    let args: Vec<String> = env::args().collect();

    // Map canary arguments to CanaryConfig struct
    let configuration = CanaryConfig {
        domain_name: &args[1],
        expiration_timer: &args[2].parse().unwrap(),
        gpg_key_id: &args[3],
        output_dir: &args[4],
    };

    // Specify path to canary txt file
    let canary_path = "./canary_elements/canary.txt";

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
    // Remove canary.txt.asc
    std::fs::remove_file(&signed_path).expect("Error removing canary.txt.asc");
    // Remove staging canary.txt
    std::fs::remove_file(&canary_path).expect("Error removing canary.txt");
    println!(
        "Canary HTML file generated successfully: {}",
        configuration.output_dir
    );
}
