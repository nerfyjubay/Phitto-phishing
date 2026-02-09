// Imports
use kuchiki::{NodeRef, parse_html, traits::*};
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};

// Local Library Imports
use crate::Errors;

// Define what the web scraper returns.
pub struct ScrapingResult {
    // The document
    pub document: NodeRef,
    // The client for reuse
    pub client: reqwest::Client,
}

// The scraping function. Input a url, get the document and a client.
pub async fn scrape_website(url: &str) -> Result<ScrapingResult, Errors> {
    // TODO: Modify this function, use playwright or a similar tool.

    // We will be using headers to avoid getting flagged as bots.
    let mut headers = HeaderMap::new(); // Declare the headermap

    // Insert the headers that we are going to use
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
             AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/121.0.0.0 Safari/537.36",
        ),
    );

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );

    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    // Build the client that will send requests to the website
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| Errors::ScrapeError(format!("Client build failed {}", e)))?; // Custom error. See errors.rs

    // Send the request and store the response
    let response = client
        .get(url) // GET method
        .send() // Send the request and await it
        .await
        .map_err(|e| Errors::ScrapeError(format!("Request failed {}", e)))?; // Custom error. See errors.rs

    // Convert the response to html text
    let html = response
        .text()
        .await
        .map_err(|e| Errors::ScrapeError(format!("Failed to read response {}", e)))?; // Custom error. See errors.rs

    // Parse the html to get the final document
    let document = parse_html().one(html);

    // END
    Ok(ScrapingResult { document, client })
}
