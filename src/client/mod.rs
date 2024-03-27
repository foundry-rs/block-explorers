pub mod blobs;
use reqwest::Client;
// The client struct which contains the base URL and the reqwest client instance.
struct BlobScanClient {
    base_url: String,
    client: Client,
}

impl BlobScanClient {
    // Initialize a new BlobScanClient.
    pub fn new(base_url: &str) -> Self {
        BlobScanClient { base_url: base_url.to_string(), client: Client::new() }
    }
}
