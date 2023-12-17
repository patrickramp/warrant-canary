// Module to scrapes news website for freshness.
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

/// Function to set app user agent.
fn get_client() -> Client {
    let agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Client::builder().user_agent(agent).build().unwrap()
}
/// Function to scrape website.
#[tokio::main]
pub(crate) async fn scrape_headlines(domain_name: &str) -> Vec<std::string::String> {
    // Define client for web request.
    let client = get_client();
    // Define url for web request.
    let url = format!("https://{}", domain_name);
    // Request result from server.
    let result = client.get(&url).send().await.unwrap();
    // Validate response.
    let raw_data = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!(
            "Something went wrong fetching site data for {}",
            domain_name
        ),
    };

    // Uncomment to save raw website html for debugging
    //save_raw_html(&raw_html, domain_name);

    // Define Vec to store contents.
    let mut headlines = Vec::new();
    // Parse response string into HTML.
    let html_data = Html::parse_document(&raw_data);
    // Define Selector to find desired content.
    let article_selector = Selector::parse("h3 > a").unwrap();

    // Iterate through site elements and isolate desired content.
    for element in html_data.select(&article_selector) {
        let headline = element.inner_html().trim().to_string();

        // Uncomment to print results to screen.
        //println!("Article: {}", headline);

        // Push headline to Vec.
        headlines.push(headline)
    }
    return headlines;
}
