// This module generates the canary HTML file from canary.txt
use crate::CanaryConfig;
use std::fs::read;
use std::io;
use std::io::Write;


// Function to compose and write out the HTML file
pub fn build_html(configuration: &CanaryConfig, canary_path: &str) -> io::Result<()> {
    // Specify name path of HTML file
    let html_name = &configuration.output_dir;
    // Open the output file for writing
    let mut html_file = std::fs::File::create(&html_name).expect("Error creating HTML file");

    // Write the header
    writeln!(html_file, "{}", header()).expect("Error writing header to HTML file");

    //Write signed canary_elements to body of HTML file
    //let html = read(String::from(("{}.asc", canary_path))
    writeln!(
        html_file,
        "{}",
        String::from_utf8_lossy(
            &read(String::from(format!("{}.asc", canary_path)))
                .expect("Error reading signed canary.txt.asc"))
    ).expect("Error writing signed canary.txt.asc to HTML file");

    // Write the HTML footer
    writeln!(html_file, "{}", footer()).expect("Error writing HTML footer to HTML file");

    Ok(())
}

// Function to write the HTML header
fn header() -> String {
    String::from(
        r###"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>
        Warrant Canary
        </title>
    </head>
    <body>
        <div style="text-align:center">
            <h1>
            Warrant Canary
            </h1>
            <hr>
        <h2>
        Please read carefully, this policy is proof of soundness.
        </h2>
        </div>
        <p>
        <pre>
        "###,
    )
}

// Function to write the HTML footer
fn footer() -> String {
    String::from(
        r###"
        </pre>
        </p>
    </body>
</html>
        "###,
    )
}
