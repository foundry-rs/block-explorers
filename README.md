# Foundry Block Explorers

Originally part of [`ethers-rs`] as `ethers-etherscan`, Foundry Block Explorers providers bindings for the [etherscan.io web API](https://docs.etherscan.io) and other block explorers.

[`ethers-rs`]: https://github.com/gakonst/ethers-rs

[![Build Status][actions-badge]][actions-url]
[![Telegram chat][telegram-badge]][telegram-url]

[actions-badge]: https://img.shields.io/github/actions/workflow/status/foundry-rs/block-explorers/ci.yml?branch=main&style=for-the-badge
[actions-url]: https://github.com/foundry-rs/block-explorers/actions?query=branch%3Amain
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[telegram-url]: https://t.me/foundry_rs

## Supported Rust Versions

<!--
When updating this, also update:
- clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

Foundry Block explorers will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.65.0.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## Examples

```rust,no_run
use ethers_core::types::Chain;
use foundry_block_explorers::Client;

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Chain::Mainnet, "<your_api_key>")?;
    // Or using environment variables
    let client = Client::new_from_env(Chain::Mainnet)?;

    let address = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".parse()?;
    let metadata = client.contract_source_code(address).await?;
    assert_eq!(metadata.items[0].contract_name, "DAO");
    Ok(())
}
```
