#![cfg(feature = "compilers-full")]

use crate::run_with_client;
use alloy_chains::Chain;
use foundry_block_explorers::verify::{VerifyContract, CodeFormat};
use foundry_compilers::{compilers::solc::{SolcCompiler, SolcLanguage}, Project, ProjectPathsConfig};
use serial_test::serial;
use std::path::Path;

#[tokio::test]
#[serial]
async fn can_flatten_and_verify_contract_single_file() {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test-data/rewards"));
    let paths = ProjectPathsConfig::builder()
        .sources(root)
        .build()
        .expect("failed to resolve project paths");
    let project = Project::builder()
        .paths(paths)
        .build(Default::default())
        .expect("failed to build the project");

    let address = "0x7777777F279eba3d3Ad8F4E708545291A6fDBA8B".parse().unwrap();
    let compiler_version = "v0.8.17+commit.8df45f5f";
    let contract = project
        .paths
        .clone()
        .with_language::<SolcLanguage>().flatten(&root.join("ProtocolRewards.sol"))
        .expect("failed to flatten contract");
    let contract_name = "ProtocolRewards.sol:ProtocolRewards".to_owned();
    let contract =
        VerifyContract::new(address, contract_name, contract, compiler_version.to_string())
            .code_format(CodeFormat::SingleFile) 
            .optimization(true)
            .runs(500000);

    run_with_client(Chain::mainnet(), |client| async move {
        let resp = client
            .submit_contract_verification(&contract)
            .await
            .expect("failed to send the request");
        // `Error!` result means that request was malformatted
        assert_ne!(resp.result, "Error!", "{resp:?}");
        assert_ne!(resp.message, "NOTOK", "{resp:?}");
    })
    .await
}

#[tokio::test]
#[serial]
async fn can_verify_contract_json_multi_file() {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test-data/rewards"));

    let paths = ProjectPathsConfig::builder()
        .sources(root)
        .build()
        .expect("failed to resolve project paths");
    let project = Project::builder()
        .paths(paths)
        .build(Default::default())
        .expect("failed to build the project");

    let address = "0x7777777F279eba3d3Ad8F4E708545291A6fDBA8B".parse().unwrap();
    let compiler_version = "v0.8.17+commit.8df45f5f";
    let contract = project
        .clone()
        .standard_json_input(&root.join("ProtocolRewards.sol"));
    let contract_name = "ProtocolRewards.sol:ProtocolRewards".to_owned();
    let contract_json_string = serde_json::to_string(&contract.expect("Contract did not compile to standard json"));
    let contract =
        VerifyContract::new(address, contract_name, contract_json_string.expect("Did not serialize json from standard json output"), compiler_version.to_string())
            .code_format(CodeFormat::SingleFile) 
            .optimization(true)
            .runs(500000);

    run_with_client(Chain::mainnet(), |client| async move {
        let resp = client
            .submit_contract_verification(&contract)
            .await
            .expect("failed to send the request");
        // `Error!` result means that request was malformatted
        assert_ne!(resp.result, "Error!", "{resp:?}");
        assert_ne!(resp.message, "NOTOK", "{resp:?}");
    })
    .await
}