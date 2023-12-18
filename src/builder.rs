// This module builds the canary.txt file
use crate::scraper::scrape_headlines;
use crate::CanaryConfig;
use chrono::Local;
use easy_hasher::easy_hasher::raw_sha256;
use regex::Regex;
use std::fs::read;
use std::io;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

// SHA-256 string hashing function
fn sha256(data: &Vec<u8>) -> String {
    let hash = raw_sha256(data.clone());
    hash.to_hex_string()
}
// Function to get the nth headline from web scrape results
fn get_headline(freshness: &Vec<String>, n: usize) -> String {
    return freshness[n].to_string();
}

// Function to parse the raw GPG signature and extract relevant information.
fn parse_fingerprint(raw_fingerprint: &str) -> String {
    // Create a regex to capture the desired part of the filename.
    let regex = Regex::new(r"(([0-9A-F]{4} *){10})").unwrap();
    // Use the regex to capture the desired part of the filename.
    let clean_signature = regex.captures(raw_fingerprint).unwrap();
    return clean_signature[1].to_string();
}

// Function to build the canary.txt file
pub fn build_canary(configuration: &CanaryConfig, canary_path: &str) -> io::Result<()> {

    // Specify domain to scrape for freshness
    let freshness_source = "www.newsnationnow.com";

    // Open the canary file for writing
    let mut canary_file = std::fs::File::create(canary_path).expect("Error creating canary.txt");

    // Set the current date and time
    let now = Local::now();

    // Write the current date and time to the canary txt file
    writeln!(
        canary_file,
        "Warrant Canary for: {}, generated on: {}",
        configuration.domain_name,
        now.format("%B %d, %Y")
    )
    .expect("Error writing current date to canary.txt");
    println!(
        "Generating warrant canary for: {} Dated: {}",
        configuration.domain_name,
        now.format("%B %d, %Y")
    );

    // Write the next canary update date to the canary file, in days from argument configuration
    let next = now + Duration::from_secs((configuration.expiration_timer) * 86400);
    writeln!(
        canary_file,
        "Next canary update on or before: {}",
        next.format("%B %d, %Y")
    )
    .expect("Error writing next canary update date to canary.txt");
    println!(
        "Next canary update on or before: {}. Dont forget to update!",
        next.format("%B %d, %Y")
    );

    // Write the policy section to the canary_elements file
    writeln!(
        canary_file,
        "{}",
        String::from_utf8_lossy(
            &read("./canary_elements/policy.txt").expect("Error reading policy file")
        )
    )
    .expect("Error writing policy to canary.txt");
    println!("Canary policy added successfully: ./canary_elements/policy.txt");

    // Write a SHA-256 policy hash to the canary file
    // Read the policy file
    let policy = read("./canary_elements/policy.txt").expect("Error reading policy file");
    // Create a SHA-256 hash of the policy
    let result = sha256(&policy);
    // Write the SHA-256 hash to the canary file
    writeln!(canary_file, "SHA256:{}", &result).expect("Error writing policy hash to canary.txt");
    println!("Policy hash added successfully: {}", result);

    // Write the announcements section to the canary_elements file
    writeln!(
        canary_file,
        "{}",
        String::from_utf8_lossy(&read("./canary_elements/announcements.txt").unwrap())
    )
    .expect("Error writing announcements to canary.txt");
    println!("Canary announcements added successfully: ./canary_elements/announcements.txt");

    // Write proof of freshness section to the canary_elements file
    writeln!(
        canary_file,
        "Headlines for today, {} from {}:\n",
        now.format("%B %d, %Y"),
        freshness_source
    )
    .expect("Error writing freshness heading to canary.txt");
    println!("Scraping freshness from {}...", freshness_source);

    // Scrape fresh headlines from freshness source
    let fresh_headlines = scrape_headlines(freshness_source);

    // Iterate through the top 3 headlines
    for headline_number in 0..=2 {
        // Print the headline to the console
        println!(
            "Headline #{}: {}",
            headline_number,
            get_headline(&fresh_headlines, headline_number)
        );
        // Write the headline to the canary_elements file
        writeln!(
            canary_file,
            ">{}",
            String::from_utf8_lossy(
                get_headline(&fresh_headlines, headline_number)
                    //.unwrap()
                    .as_ref()
            )
        )
        .expect("Error writing freshness to canary.txt");
    }
    println!("Canary freshness updated successfully.");

    // Write the signers section to the canary_elements file
    // Collect raw signers GPG fingerprint
    let fingerprint = Command::new("gpg")
        .args(["--fingerprint", &configuration.gpg_key_id])
        .output()
        .expect("failed to get fingerprint");
    let raw_fingerprint = String::from_utf8_lossy(&fingerprint.stdout).to_string();

    // Use regex to extract the GPG fingerprint from the raw fingerprint
    let clean_fingerprint = parse_fingerprint(&raw_fingerprint);
    // Write the GPG fingerprint to the canary_elements file
    writeln!(
        canary_file,
        "\n=======================================================================\n\
Signed:\n\n - {} \n{}\n",
        &configuration.gpg_key_id,
        clean_fingerprint.to_uppercase()
    )
    .expect("Error writing signers to canary.txt");
    println!(
        "Signer fingerprint added successfully: {}: {}",
        &configuration.gpg_key_id, &clean_fingerprint
    );
    println!("New canary.txt file generated successfully: {}", canary_path);

    Ok(())
}
