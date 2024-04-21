#![doc = include_str!("../README.md")]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use alloy_chains::{Chain, ChainKind, NamedChain};
use alloy_rpc_types::BlockHashOrNumber;
pub use request::*;
pub use response::*;
use serde::de::DeserializeOwned;

pub mod request;
pub mod response;

/// Client for [Blobscan API](https://api.blobscan.com/)
///
/// See also [Blobscan API Documentation](https://docs.blobscan.com/)
#[derive(Clone, Debug)]
pub struct Client {
    /// Base URL for the API
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal [reqwest::Client]
    pub fn new(baseurl: impl Into<String>) -> Self {
        let client = {
            let dur = std::time::Duration::from_secs(30);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Create a new client instance for `blobscan.com` with the correct endpoint based on the
    /// chain.
    ///
    /// At this time, only the following chains are supported by Blobscan:
    /// - Ethereum Mainnet: <https://api.blobscan.com/>
    /// - Sepolia Testnet: <https://api.sepolia.blobscan.com/>
    /// - Holesky Testnet: <https://api.holesky.blobscan.com/>
    ///
    /// For other chains this will return `None`
    pub fn new_chain(chain: Chain) -> Option<Self> {
        Self::new_chain_with_client(chain, reqwest::Client::new())
    }

    /// Create a new client instance for `blobscan.com` with the given [reqwest::Client] and the
    /// correct endpoint based on the chain.
    ///
    /// At this time, only the following chains are supported by Blobscan:
    /// - Ethereum Mainnet: <https://api.blobscan.com/>
    /// - Sepolia Testnet: <https://api.sepolia.blobscan.com/>
    /// - Holesky Testnet: <https://api.holesky.blobscan.com/>
    ///
    /// For other chains this will return `None`
    pub fn new_chain_with_client(chain: Chain, client: reqwest::Client) -> Option<Self> {
        match chain.kind() {
            ChainKind::Named(NamedChain::Mainnet) => {
                Some(Self::new_with_client("https://api.blobscan.com/", client))
            }
            ChainKind::Named(NamedChain::Sepolia) | ChainKind::Named(NamedChain::Holesky) => {
                Some(Self::new_with_client(format!("https://api.{chain}.blobscan.com/"), client))
            }
            _ => None,
        }
    }

    /// Creates a new client instance for the Ethereum Mainnet with the correct endpoint: <https://api.blobscan.com/>
    pub fn mainnet() -> Option<Self> {
        Self::new_chain(Chain::mainnet())
    }

    /// Creates a new client instance for the sepolia Testnet with the correct endpoint: <https://api.sepolia.blobscan.com/>
    pub fn sepolia() -> Option<Self> {
        Self::new_chain(Chain::sepolia())
    }

    /// Creates a new client instance for the holesky Testnet with the correct endpoint: <https://api.holesky.blobscan.com/>
    pub fn holesky() -> Option<Self> {
        Self::new_chain(Chain::holesky())
    }

    /// Creates a new client instance for the Ethereum Mainnet with the given [reqwest::Client] and  the correct endpoint: <https://api.blobscan.com/>
    pub fn mainnet_with_client(client: reqwest::Client) -> Option<Self> {
        Self::new_chain_with_client(Chain::mainnet(), client)
    }

    /// Creates a new client instance for the sepolia Testnet with the given [reqwest::Client] and  the correct endpoint: <https://api.sepolia.blobscan.com/>
    pub fn sepolia_with_client(client: reqwest::Client) -> Option<Self> {
        Self::new_chain_with_client(Chain::sepolia(), client)
    }

    /// Creates a new client instance for the holesky Testnet with the given [reqwest::Client] and the correct endpoint: <https://api.holesky.blobscan.com/>
    pub fn holesky_with_client(client: reqwest::Client) -> Option<Self> {
        Self::new_chain_with_client(Chain::holesky(), client)
    }

    /// Construct a new client with an existing [reqwest::Client] allowing more control over its
    /// configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    pub fn new_with_client(baseurl: impl Into<String>, client: reqwest::Client) -> Self {
        Self { baseurl: baseurl.into(), client }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    async fn get_block<T: DeserializeOwned>(
        &self,
        block: BlockHashOrNumber,
        query: GetBlockQuery,
    ) -> reqwest::Result<T> {
        self.client
            .get(&format!("{}/{}", self.baseurl, block))
            .query(&query)
            .send()
            .await?
            .json()
            .await
    }

    /// Retrieves the __full__ block details for given block number or hash.
    ///
    /// This
    ///
    /// Sends a `GET` request to `/blocks/{id}`
    pub async fn block(
        &self,
        block: BlockHashOrNumber,
    ) -> reqwest::Result<BlockDetails<FullTransactionDetails>> {
        self.get_block(block, GetBlockQuery::default()).await
    }

    /// Retrieves the specific block details for given block number or hash.
    ///
    /// Sends a `GET` request to `/blocks/{id}`
    pub async fn block_with_query(
        &self,
        block: BlockHashOrNumber,
        query: GetBlockQuery,
    ) -> reqwest::Result<BlockDetails<SelectedTransactionDetails>> {
        self.get_block(block, query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_block_by_id() {}
}
