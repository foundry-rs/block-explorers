use std::env;

use crate::run_with_client;
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{U256, U64};
use foundry_block_explorers::{
    account::{InternalTxQueryOption, TokenQueryOption},
    block_number::BlockNumber,
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn get_ether_balance_single_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let balance = client
            .get_ether_balance_single(
                &"0x58eB28A67731c570Ef827C365c89B5751F9E6b0a".parse().unwrap(),
                None,
            )
            .await;
        balance.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_ether_balance_multi_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let balances = client
            .get_ether_balance_multi(
                &["0x58eB28A67731c570Ef827C365c89B5751F9E6b0a".parse().unwrap()],
                None,
            )
            .await;
        assert!(balances.is_ok());
        let balances = balances.unwrap();
        assert_eq!(balances.len(), 1);
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_transactions_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_transactions(&"0x4F26FfBe5F04ED43630fdC30A87638d53D0b0876".parse().unwrap(), None)
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_internal_transactions_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_internal_transactions(
                InternalTxQueryOption::ByAddress(
                    "0x2c1ba59d6f58433fb1eaee7d20b26ed83bda51a3".parse().unwrap(),
                ),
                None,
            )
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_internal_transactions_by_tx_hash_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_internal_transactions(
                InternalTxQueryOption::ByTransactionHash(
                    "0x40eb908387324f2b575b4879cd9d7188f69c8fc9d87c901b9e2daaea4b442170"
                        .parse()
                        .unwrap(),
                ),
                None,
            )
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_erc20_transfer_events_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_erc20_token_transfer_events(
                TokenQueryOption::ByAddress(
                    "0x4e83362442b8d1bec281594cea3050c8eb01311c".parse().unwrap(),
                ),
                None,
            )
            .await
            .unwrap();
        let tx = txs.first().unwrap();
        assert_eq!(tx.gas_used, U256::from(93657u64));
        assert_eq!(tx.nonce, U256::from(10u64));
        assert_eq!(tx.block_number, BlockNumber::Number(U64::from(2228258u64)));
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_erc721_transfer_events_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_erc721_token_transfer_events(
                TokenQueryOption::ByAddressAndContract(
                    "0x6975be450864c02b4613023c2152ee0743572325".parse().unwrap(),
                    "0x06012c8cf97bead5deae237070f9587f8e7a266d".parse().unwrap(),
                ),
                None,
            )
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_erc1155_transfer_events_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        let txs = client
            .get_erc1155_token_transfer_events(
                TokenQueryOption::ByAddressAndContract(
                    "0x216CD350a4044e7016f14936663e2880Dd2A39d7".parse().unwrap(),
                    "0x495f947276749ce646f68ac8c248420045cb7b5e".parse().unwrap(),
                ),
                None,
            )
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_mined_blocks_success() {
    run_with_client(Chain::mainnet(), |client| async move {
        client
            .get_mined_blocks(
                &"0x9dd134d14d1e65f84b706d6f205cd5b1cd03a46b".parse().unwrap(),
                None,
                None,
            )
            .await
            .unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_avalanche_transactions() {
    run_with_client(Chain::from_named(NamedChain::Avalanche), |client| async move {
        let txs = client
            .get_transactions(&"0x1549ea9b546ba9ffb306d78a1e1f304760cc4abf".parse().unwrap(), None)
            .await;
        txs.unwrap();
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_etherscan_polygon_key_v2() {
    // This requires the etherscan api key to be set – expected for this test suite.
    let etherscan_test_api_key = env::var("ETHERSCAN_API_KEY").unwrap();
    env::set_var("POLYGONSCAN_API_KEY", "POLYGONSCAN1");

    run_with_client(Chain::from_named(NamedChain::Polygon), |client| async move {
        assert_eq!(client.api_key().unwrap(), etherscan_test_api_key);

        env::set_var("POLYGONSCAN_API_KEY", "");
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_fantom_key_ftmscan() {
    env::set_var("FMTSCAN_API_KEY", "FANTOMSCAN1");

    run_with_client(Chain::from_named(NamedChain::Fantom), |client| async move {
        assert_eq!(client.api_key().unwrap(), "FANTOMSCAN1");

        env::set_var("FMTSCAN_API_KEY", "");
    })
    .await
}

#[tokio::test]
#[serial]
async fn get_fantom_key_fantomscan() {
    env::set_var("FANTOMSCAN_API_KEY", "FANTOMSCAN2");

    run_with_client(Chain::from_named(NamedChain::Fantom), |client| async move {
        assert_eq!(client.api_key().unwrap(), "FANTOMSCAN2");

        env::set_var("FANTOMSCAN_API_KEY", "");
    })
    .await
}

#[cfg(test)]
mod internal_transaction_tests {
    use alloy_primitives::{Address, B256, U256};
    use foundry_block_explorers::{
        account::{GenesisOption, InternalTransaction},
        block_number::BlockNumber,
    };

    #[test]
    fn test_internal_transaction_serialization_deserialization() {
        let contract_addr: Address = "0x0a36f9565c6fb862509ad8d148941968344a55d8".parse().unwrap();
        let from_addr: Address = "0x4dadacd4aaa54c68c715f70c05a8e873ef9bb0a8".parse().unwrap();
        let hash: B256 =
            "0xb349ce8f75676f186eb5e6427b72b74da55d5b70b70e5fee5b3a804a302796cc".parse().unwrap();

        let internal_tx = InternalTransaction {
            block_number: BlockNumber::Pending,
            time_stamp: "0".to_string(),
            hash,
            from: from_addr,
            to: GenesisOption::None,
            value: U256::ZERO,
            contract_address: GenesisOption::Some(contract_addr),
            input: GenesisOption::None,
            result_type: "create".to_string(),
            gas: U256::from(4438777u64),
            gas_used: U256::from(3209972u64),
            trace_id: "0_1_1".to_string(),
            is_error: "0".to_string(),
            err_code: "".to_string(),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&internal_tx).expect("Failed to serialize");

        // Check that the JSON does NOT contain escaped quotes (the bug symptom)
        assert!(
            !json.contains("\\\""),
            "JSON contains escaped quotes, indicating double serialization bug still exists"
        );

        // The contract address should appear in the JSON as a proper address, not as an escaped
        // string
        assert!(
            json.contains("0x0a36f9565c6fb862509ad8d148941968344a55d8"),
            "Contract address not found in JSON"
        );

        // Deserialize from JSON - this should work without panicking
        let deserialized: InternalTransaction =
            serde_json::from_str(&json).expect("Failed to deserialize - the fix didn't work");

        // Verify the round-trip worked correctly
        match (&internal_tx.contract_address, &deserialized.contract_address) {
            (GenesisOption::Some(original), GenesisOption::Some(deserialized)) => {
                assert_eq!(original, deserialized);
            }
            (a, b) => panic!("Contract address mismatch: {:?} != {:?}", a, b),
        }
        assert_eq!(internal_tx.hash, deserialized.hash);
        assert_eq!(internal_tx.from, deserialized.from);
        assert_eq!(internal_tx.gas, deserialized.gas);
    }
}
