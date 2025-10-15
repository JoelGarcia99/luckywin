use reqwest::{
    Client,
    multipart::{Form, Part},
};

/// Sends HTML content to Gotenberg and receives a PDF back
///
/// # Arguments
/// * `html` - The HTML content to convert to PDF
/// * `client` - The HTTP client to use for the request
///
/// # Returns
/// * `Ok(Vec<u8>)` - The PDF bytes on success
/// * `Err(())` - On any error during conversion
pub async fn send_html_to_gotenberg(html: &str, client: &Client) -> Result<Vec<u8>, ()> {
    // Build a multipart part with the HTML content
    let part = match Part::text(html.to_owned())
        .file_name("index.html")
        .mime_str("text/html")
    {
        Ok(part) => part,
        Err(err) => {
            tracing::error!("Failed to create multipart part: {:?}", err);
            return Err(());
        }
    };

    // Construct the multipart form with the field "files" as expected by Gotenberg
    let form = Form::new().part("files", part);

    // Make the POST request with the multipart form
    let response = match client
        .post("http://gotenberg:3000/forms/chromium/convert/html")
        .multipart(form)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Gotenberg request failed: {:?}", err);
            return Err(());
        }
    };

    // Check for a successful response and extract the PDF bytes
    if response.status().is_success() {
        let pdf_bytes = match response.bytes().await {
            Ok(pdf_bytes) => pdf_bytes,
            Err(err) => {
                tracing::error!("Failed to read PDF bytes: {:?}", err);
                return Err(());
            }
        };
        Ok(pdf_bytes.to_vec())
    } else {
        tracing::error!(
            "Gotenberg returned non-success status: {}",
            response.status()
        );
        Err(())
    }
}
