// Imports
use kuchiki::NodeRef;
use reqwest::Client;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use url::Url;

// Local Library Imports
use crate::Errors;

// This function will copy the resources (images, etc) from the page and store them locally.
// Input the document to copy from, the client, the url and the target directory for the files.
pub async fn copy_resources(
    document: &NodeRef,
    client: &Client,
    base_url: &str,
    target_dir: &str,
) -> Result<(), Errors> {
    // Create target directories
    fs::create_dir_all(target_dir)
        .await
        .map_err(|e| Errors::CopyAssetError(format!("Unable to create target directory: {}", e)))?;

    let static_dir = Path::new(target_dir).join("static");
    fs::create_dir_all(&static_dir)
        .await
        .map_err(|e| Errors::CopyAssetError(format!("Unable to create static directory: {}", e)))?;

    let mut attr_updates = Vec::new();

    // Use css to gather the resources (links, scripts, imgs)
    for css_match in document
        .select("link, script, img")
        .map_err(|_| Errors::CopyAssetError(format!("Unable to select resources")))?
    {
        // Convert links to href and scripts or images to src tags.
        let element_ref = css_match.as_node().as_element();
        if let Some(element) = element_ref {
            let tag_name = element.name.local.to_string();
            let attr = match tag_name.as_str() {
                "link" => "href",
                "script" | "img" => "src",
                _ => continue,
            };

            if let Some(resource_url) = element.attributes.borrow().get(attr) {
                // Store what needs updating (no borrowing here)
                attr_updates.push((element.clone(), attr.to_string(), resource_url.to_string()));
            }
        }
    }

    // Process downloads and updates
    for (element, attr, resource_url) in attr_updates {
        // Resolve relative URLs
        let full_url = if resource_url.starts_with("http") {
            resource_url
        } else {
            Url::parse(base_url)
                .and_then(|base| base.join(&resource_url))
                .map_err(|e| {
                    Errors::CopyAssetError(format!("Invalid URL {}: {}", resource_url, e))
                })?
                .to_string()
        };

        // Determine resource file name (strip query string)
        let resource_name = Path::new(full_url.split('?').next().unwrap())
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                Errors::CopyAssetError(format!("Invalid resource name: {}", full_url))
            })?;

        let resource_path = static_dir.join(resource_name);

        if resource_path.is_dir() {
            continue;
        }

        // Fetch the resource
        let content = client
            .get(&full_url)
            .send()
            .await
            .map_err(|e| Errors::CopyAssetError(format!("Failed to GET {}: {}", full_url, e)))?
            .bytes()
            .await
            .map_err(|e| {
                Errors::CopyAssetError(format!("Failed to read bytes from {}: {}", full_url, e))
            })?;

        // Write to local static file
        let mut file = fs::File::create(&resource_path).await.map_err(|e| {
            Errors::CopyAssetError(format!("Failed to create file {:?}: {}", resource_path, e))
        })?;
        file.write_all(&content).await.map_err(|e| {
            Errors::CopyAssetError(format!(
                "Failed to write to file {:?}: {}",
                resource_path, e
            ))
        })?;

        // Now update attributes
        let mut attrs = element.attributes.borrow_mut();
        attrs.insert(attr, format!("/static/{}", resource_name));
    }

    // END
    Ok(())
}
