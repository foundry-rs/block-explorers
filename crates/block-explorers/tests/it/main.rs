//! Etherscan integration tests

#![cfg(not(target_arch = "wasm32"))]

use alloy_chains::{Chain, ChainKind, NamedChain};
use foundry_block_explorers::{errors::EtherscanError, Client};
use std::{
    env,
    future::Future,
    path::PathBuf,
    time::{Duration, Instant},
};

mod account;
mod blocks;
mod contract;
mod gas;
mod transaction;
#[cfg(feature = "foundry-compilers")]
mod verify;
mod version;

/// Calls the function with a new Etherscan Client.
pub async fn run_with_client<F, Fut, T>(chain: Chain, f: F) -> T
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = T>,
{
    init_tracing();
    let (client, duration) = match Client::new_from_env(chain) {
        Ok(c) => (c, rate_limit(chain, true)),
        Err(_) => {
            (Client::builder().chain(chain).unwrap().build().unwrap(), rate_limit(chain, false))
        }
    };
    run_at_least_duration(duration, f(client)).await
}

/// Calls the function with a new Etherscan Client.
pub async fn run_with_client_v2<F, Fut, T>(chain: Chain, f: F) -> T
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = T>,
{
    init_tracing();
    let (client, duration) = match Client::new_v2_from_env(chain) {
        Ok(c) => (c, rate_limit(chain, true)),
        Err(_) => (
            Client::builder()
                .with_api_version(foundry_block_explorers::EtherscanApiVersion::V2)
                .chain(chain)
                .unwrap()
                .build()
                .unwrap(),
            rate_limit(chain, false),
        ),
    };
    run_at_least_duration(duration, f(client)).await
}

/// Calls the function with a new cached Etherscan Client.
pub async fn run_with_client_cached<F, Fut, T>(chain: Chain, f: F) -> T
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = T>,
{
    init_tracing();
    let cache_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-data/cache");

    let api_key = match chain.kind() {
        ChainKind::Named(named) => match named {
            // Extra aliases
            NamedChain::Fantom | NamedChain::FantomTestnet => std::env::var("FMTSCAN_API_KEY")
                .or_else(|_| std::env::var("FANTOMSCAN_API_KEY"))
                .map_err(Into::into),

            // Backwards compatibility, ideally these should return an error.
            NamedChain::Gnosis
            | NamedChain::Chiado
            | NamedChain::Sepolia
            | NamedChain::Rsk
            | NamedChain::Sokol
            | NamedChain::Poa
            | NamedChain::Oasis
            | NamedChain::Emerald
            | NamedChain::EmeraldTestnet
            | NamedChain::Evmos
            | NamedChain::EvmosTestnet => Ok(String::new()),
            NamedChain::AnvilHardhat | NamedChain::Dev => {
                Err(EtherscanError::LocalNetworksNotSupported)
            }

            _ => named
                .etherscan_api_key_name()
                .ok_or_else(|| EtherscanError::ChainNotSupported(chain))
                .and_then(|key_name| std::env::var(key_name).map_err(Into::into)),
        },
        ChainKind::Id(_) => Err(EtherscanError::ChainNotSupported(chain)),
    }
    .unwrap();

    let (client, duration) = match Client::builder()
        .with_api_key(api_key)
        .chain(chain)
        .unwrap()
        .with_cache(Some(cache_path), Duration::from_secs(24 * 60 * 60))
        .with_api_key(env::var("ETHERSCAN_API_KEY").unwrap())
        .build()
    {
        Ok(c) => (c, rate_limit(chain, true)),
        Err(_) => {
            (Client::builder().chain(chain).unwrap().build().unwrap(), rate_limit(chain, false))
        }
    };
    run_at_least_duration(duration, f(client)).await
}

#[track_caller]
fn rate_limit(chain: Chain, key: bool) -> Duration {
    match (chain.kind(), key) {
        // Rate limit with an API key is 5 call per second.
        (_, true) => Duration::from_millis(250),

        // Rate limit without an API key is 1 call every 5 seconds.
        // (ChainKind::Named(NamedChain::Mainnet), false) => Duration::from_millis(5100),
        (ChainKind::Named(NamedChain::Mainnet), false) => panic!("ETHERSCAN_API_KEY is not set"),

        // Ignore other chains since we don't have more than 1 test with each.
        (_, false) => Duration::ZERO,
    }
}

async fn run_at_least_duration<T>(duration: Duration, block: impl Future<Output = T>) -> T {
    let start = Instant::now();
    let output = block.await;
    let elapsed = start.elapsed();
    if elapsed < duration {
        tokio::time::sleep(duration - elapsed).await;
    }
    output
}

#[track_caller]
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

#[tokio::test]
async fn check_wrong_etherscan_api_key() {
    let client = Client::new(Chain::mainnet(), "ABCDEFG").unwrap();
    let resp = client
        .contract_source_code("0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".parse().unwrap())
        .await
        .unwrap_err();

    assert!(matches!(resp, EtherscanError::InvalidApiKey));
}
