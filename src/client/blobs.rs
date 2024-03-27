use super::BlobScanClient;
use alloy_chains::Chain;
use serde::{ser::StdError, Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlobListResponse {
    blobs: Vec<Blob>,
    #[serde(rename = "totalBlobs")]
    total_blobs: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blob {
    #[serde(rename = "versionedHash")]
    versioned_hash: String,
    commitment: String,
    proof: String,
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlobId {
    #[serde(rename = "versionedHash")]
    versioned_hash: String,
    commitment: String,
    proof: String,
    size: u32,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    message: String,
    code: String,
    issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    message: String,
}

//
// #[derive(Debug)]
// pub struct BlobQueryParams {
//     pub rollup: Option<String>,
//     pub page: Option<u32>,
//     pub page_size: Option<u32>,
// }
#[derive(Debug, Clone, Copy, Default)]
pub struct BlobQueryParams {
    pub rollup: Option<Chain>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
impl BlobScanClient {
    // Retrieve  blobs.
    pub async fn get_blobs(
        &self,
        params: BlobQueryParams,
    ) -> Result<BlobListResponse, BlobScanError> {
        let url = format!("{}/blobs", self.base_url);
        let mut request = self.client.get(url);

        
        if let Some(rollup) = params.rollup {
            // request = request.query(&[("rollup", rollup)]);
            request = request.query(&[("rollup", rollup.to_string())]);
        }
        if let Some(page) = params.page {
            request = request.query(&[("p", page.to_string())]);
        }
        if let Some(page_size) = params.page_size {
            request = request.query(&[("ps", page_size.to_string())]);
        }

        let response = request.send().await?;

        
        match response.status().is_success() {
            true => {
                let blob_list = response.json::<BlobListResponse>().await?;
                Ok(blob_list)
            }
            false => {
                let api_error = response.json::<ApiError>().await?;
                Err(BlobScanError::ApiError(api_error))
            }
        }
    }

    // Retrieve details for a specific blob by ID.
    pub async fn get_blob_by_id(&self, id: &str) -> Result<BlobId, BlobScanError> {
        let url = format!("{}/blobs/{}", self.base_url, id);
        let response = self.client.get(url).send().await?;

        // Check the status code of the response, parsing errors if necessary.
        match response.status().is_success() {
            true => {
                let blob = response.json::<BlobId>().await?;
                Ok(blob)
            }
            false => {
                let api_error = response.json::<ApiError>().await?;
                Err(BlobScanError::ApiError(api_error))
            }
        }
    }
}

#[tokio::test]
async fn testing() {
    let client = BlobScanClient::new("https://api.blobscan.com");

    // Create query parameters.
    let params =
        BlobQueryParams { rollup: Some(Chain::base_mainnet()), page: Some(1), page_size: Some(10) };

    match client.get_blobs(params).await {
        Ok(response) => {
            // println!("Total Blobs: {}", response.total_blobs);
            println!("Blobs retrieved {:#?}", response);
            // println!(
            //     "Blobs retrieved {:?}",
            //     response.blobs.iter().map(|f| { f.versioned_hash.clone() })
            // );
        }
        Err(e) => {
            eprintln!("Failed to retrieve blobs: {}", e);
        }
    }
}
#[tokio::test]
async fn teeeest() {
    let client = BlobScanClient::new("https://api.blobscan.com");

    match client
        .get_blob_by_id("0x010000324a070ee49e3cac7708f17c7682dc154be8328cf74ae7f45da12e708a")
        .await
    {
        Ok(blob) => {
            println!("Blob details : {:#?}", blob);
        }
        Err(e) => {
            eprintln!("Failed to retrieve blobs: {}", e);
        }
    }
}

#[derive(Debug)]
pub enum BlobScanError {
    ReqwestError(reqwest::Error),
    ApiError(ApiError),
}

// Implement Display for BlobScanError.
impl fmt::Display for BlobScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlobScanError::ReqwestError(e) => write!(f, "Request error: {}", e),
            BlobScanError::ApiError(e) => write!(f, "API error: {}", e.message),
        }
    }
}

impl StdError for BlobScanError {}

impl From<reqwest::Error> for BlobScanError {
    fn from(error: reqwest::Error) -> Self {
        BlobScanError::ReqwestError(error)
    }
}

impl From<ApiError> for BlobScanError {
    fn from(error: ApiError) -> Self {
        BlobScanError::ApiError(error)
    }
}
