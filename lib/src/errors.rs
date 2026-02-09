use thiserror::Error;

// Define the different errors that we will use in our code for debugging.
// TODO: Improve error handling and add debugging
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Failed to scrape website: {0}")]
    ScrapeError(String),

    #[error("Encountered an error when copying the assets: {0}")]
    CopyAssetError(String),

    #[error("Failed to retrieve and manage forms: {0}")]
    FormError(String),
}
