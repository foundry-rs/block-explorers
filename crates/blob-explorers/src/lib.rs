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
use alloy_primitives::B256;
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
    pub fn mainnet() -> Self {
        Self::new_chain(Chain::mainnet()).unwrap()
    }

    /// Creates a new client instance for the sepolia Testnet with the correct endpoint: <https://api.sepolia.blobscan.com/>
    pub fn sepolia() -> Self {
        Self::new_chain(Chain::sepolia()).unwrap()
    }

    /// Creates a new client instance for the holesky Testnet with the correct endpoint: <https://api.holesky.blobscan.com/>
    pub fn holesky() -> Self {
        Self::new_chain(Chain::holesky()).unwrap()
    }

    /// Creates a new client instance for the Ethereum Mainnet with the given [reqwest::Client] and  the correct endpoint: <https://api.blobscan.com/>
    pub fn mainnet_with_client(client: reqwest::Client) -> Self {
        Self::new_chain_with_client(Chain::mainnet(), client).unwrap()
    }

    /// Creates a new client instance for the sepolia Testnet with the given [reqwest::Client] and  the correct endpoint: <https://api.sepolia.blobscan.com/>
    pub fn sepolia_with_client(client: reqwest::Client) -> Self {
        Self::new_chain_with_client(Chain::sepolia(), client).unwrap()
    }

    /// Creates a new client instance for the holesky Testnet with the given [reqwest::Client] and the correct endpoint: <https://api.holesky.blobscan.com/>
    pub fn holesky_with_client(client: reqwest::Client) -> Self {
        Self::new_chain_with_client(Chain::holesky(), client).unwrap()
    }

    /// Construct a new client with an existing [reqwest::Client] allowing more control over its
    /// configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    pub fn new_with_client(baseurl: impl Into<String>, client: reqwest::Client) -> Self {
        let mut baseurl = baseurl.into();
        if !baseurl.ends_with('/') {
            baseurl.push('/');
        }
        Self { baseurl, client }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    async fn get_transaction<T: DeserializeOwned>(
        &self,
        hash: B256,
        query: GetTransactionQuery,
    ) -> reqwest::Result<T> {
        self.client
            .get(&format!("{}transactions/{}", self.baseurl, hash))
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&query)
            .send()
            .await?
            .json()
            .await
    }

    /// Retrieves the __full__ transaction details for given block transaction hash.
    ///
    /// This
    ///
    /// Sends a `GET` request to `/transactions/{hash}`
    pub async fn transaction(&self, tx_hash: B256) -> reqwest::Result<TransactionDetails> {
        self.get_transaction(tx_hash, Default::default()).await
    }

    /// Retrieves the specific transaction details for given transaction hash.
    ///
    /// Sends a `GET` request to `/transactions/{hash}`
    pub async fn transaction_with_query(
        &self,
        tx_hash: B256,
        query: GetTransactionQuery,
    ) -> reqwest::Result<TransactionDetails> {
        self.get_transaction(tx_hash, query).await
    }

    async fn get_block<T: DeserializeOwned>(
        &self,
        block: BlockHashOrNumber,
        query: GetBlockQuery,
    ) -> reqwest::Result<T> {
        self.client
            .get(&format!("{}blocks/{}", self.baseurl, block))
            .header(reqwest::header::ACCEPT, "application/json")
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
    ) -> reqwest::Result<BlockResponse<FullTransactionDetails>> {
        self.get_block(block, GetBlockQuery::default()).await
    }

    /// Retrieves the specific block details for given block number or hash.
    ///
    /// Sends a `GET` request to `/blocks/{id}`
    pub async fn block_with_query(
        &self,
        block: BlockHashOrNumber,
        query: GetBlockQuery,
    ) -> reqwest::Result<BlockResponse<SelectedTransactionDetails>> {
        self.get_block(block, query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_block_by_id() {
        let block = "0xc3a0113f60107614d1bba950799903dadbc2116256a40b1fefb37e9d409f1866";
        let client = Client::holesky();

        let _block = client.block(block.parse().unwrap()).await.unwrap();
    }

    #[tokio::test]
    async fn get_single_transaction() {
        let tx = "0xd4f136048a56b9b62c9cdca0ce0dbb224295fd0e0170dbbc78891d132f639d60";
        let client = Client::holesky();

        let tx = client.transaction(tx.parse().unwrap()).await.unwrap();
        dbg!(&tx);
    }
}
