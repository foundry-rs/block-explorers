# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.22.0](https://github.com/foundry-rs/block-explorers/releases/tag/v0.22.0) - 2025-09-01

### Bug Fixes

- [serde] Support 0x-prefixed hex in deserialize_stringified_block_number ([#100](https://github.com/foundry-rs/block-explorers/issues/100))
- [tests] Disambiguate `Into` target for `parse_units` in `gas.rs` ([#98](https://github.com/foundry-rs/block-explorers/issues/98))
- `dependencies.yml` is unused ([#104](https://github.com/foundry-rs/block-explorers/issues/104))

### Dependencies

- [deps] Bump compilers 0.19.0 ([#107](https://github.com/foundry-rs/block-explorers/issues/107))
- [deps] Add dependencies ci workflow + update deps + fix clippy ([#102](https://github.com/foundry-rs/block-explorers/issues/102))

### Miscellaneous Tasks

- Deprecate Etherscan V1 ([#101](https://github.com/foundry-rs/block-explorers/issues/101))
- Release 0.21.0 ([#108](https://github.com/foundry-rs/block-explorers/issues/108))
- Add @0xrusowsky to `CODEOWNERS` ([#105](https://github.com/foundry-rs/block-explorers/issues/105))
- Update `CODEOWNERS` to improve visibility ([#103](https://github.com/foundry-rs/block-explorers/issues/103))

## [foundry-block-explorers-v0.20.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.20.0) - 2025-07-14

### Dependencies

- Bump solar + MSRV ([#96](https://github.com/foundry-rs/block-explorers/issues/96))

### Miscellaneous Tasks

- Release 0.20.0
- Add trace for getabi ([#92](https://github.com/foundry-rs/block-explorers/issues/92))
- Update CI flow, add workflow_dispatch, remove unused GOERLI_PRIVATE_KEY ([#95](https://github.com/foundry-rs/block-explorers/issues/95))

## [foundry-block-explorers-v0.19.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.19.1) - 2025-07-03

### Bug Fixes

- Etherscan V2 API URLs from `alloy-chains` already contain `chainid` ([#93](https://github.com/foundry-rs/block-explorers/issues/93))

### Miscellaneous Tasks

- Release 0.19.1

## [foundry-block-explorers-v0.19.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.19.0) - 2025-06-30

### Miscellaneous Tasks

- Release 0.19.0
- Rustmft

### Other

- Support vyper-json codeformat ([#91](https://github.com/foundry-rs/block-explorers/issues/91))

## [foundry-block-explorers-v0.18.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.18.0) - 2025-05-29

### Dependencies

- Bump compilers v0.17.0 ([#90](https://github.com/foundry-rs/block-explorers/issues/90))

### Miscellaneous Tasks

- Release 0.18.0

## [foundry-block-explorers-v0.17.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.17.0) - 2025-05-16

### Dependencies

- Alloy 1.0 ([#89](https://github.com/foundry-rs/block-explorers/issues/89))

### Miscellaneous Tasks

- Release 0.17.0

## [foundry-block-explorers-v0.16.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.16.0) - 2025-05-12

### Dependencies

- Bump compilers to 0.16.0 ([#88](https://github.com/foundry-rs/block-explorers/issues/88))

### Miscellaneous Tasks

- Release 0.16.0
- Release 0.15.0

## [foundry-block-explorers-v0.14.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.14.0) - 2025-05-07

### Dependencies

- [deps] Bump alloy-core 1.0 + alloy 0.15 ([#86](https://github.com/foundry-rs/block-explorers/issues/86))

### Miscellaneous Tasks

- Release 0.14.0

## [foundry-block-explorers-v0.13.3](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.13.3) - 2025-05-07

### Miscellaneous Tasks

- Release 0.13.3
- FromStr for EtherscanApiVersion ([#85](https://github.com/foundry-rs/block-explorers/issues/85))

## [foundry-block-explorers-v0.13.2](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.13.2) - 2025-05-05

### Miscellaneous Tasks

- Release 0.13.2
- Allow CDLA-Permissive-2.0 ([#84](https://github.com/foundry-rs/block-explorers/issues/84))

### Other

- Handle parsing and serialization of etherscan api version ([#83](https://github.com/foundry-rs/block-explorers/issues/83))

## [foundry-block-explorers-v0.13.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.13.1) - 2025-04-15

### Miscellaneous Tasks

- Release 0.13.1

### Other

- Update etherscan lib to handle both GET and POST parameters for chainid ([#82](https://github.com/foundry-rs/block-explorers/issues/82))

## [foundry-block-explorers-v0.13.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.13.0) - 2025-04-07

### Dependencies

- Bump compilers to v0.14 ([#81](https://github.com/foundry-rs/block-explorers/issues/81))

### Miscellaneous Tasks

- Release 0.13.0

## [foundry-block-explorers-v0.12.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.12.0) - 2025-04-02

### Dependencies

- Bump alloy 0.13 ([#80](https://github.com/foundry-rs/block-explorers/issues/80))

### Miscellaneous Tasks

- Release 0.12.0

## [foundry-block-explorers-v0.11.2](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.11.2) - 2025-03-15

### Miscellaneous Tasks

- Release 0.11.2

### Other

- Add v2 verify routes ([#73](https://github.com/foundry-rs/block-explorers/issues/73))

## [foundry-block-explorers-v0.11.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.11.1) - 2025-03-15

### Bug Fixes

- Fix tests ([#77](https://github.com/foundry-rs/block-explorers/issues/77))

### Dependencies

- Bump alloy 0.12 ([#79](https://github.com/foundry-rs/block-explorers/issues/79))
- Bump alloy 0.11 ([#76](https://github.com/foundry-rs/block-explorers/issues/76))

### Miscellaneous Tasks

- Release 0.11.1
- Allow paste ([#78](https://github.com/foundry-rs/block-explorers/issues/78))

## [foundry-block-explorers-v0.11.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.11.0) - 2025-01-21

### Dependencies

- Bump compilers ([#74](https://github.com/foundry-rs/block-explorers/issues/74))

### Miscellaneous Tasks

- Release 0.11.0
- Update deny.toml ([#71](https://github.com/foundry-rs/block-explorers/issues/71))

### Other

- Move deny to ci ([#70](https://github.com/foundry-rs/block-explorers/issues/70))

## [foundry-blob-explorers-v0.10.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-blob-explorers-v0.10.0) - 2024-12-09

### Dependencies

- Bump alloy 0.7 ([#69](https://github.com/foundry-rs/block-explorers/issues/69))

### Miscellaneous Tasks

- Release 0.10.0
- Release 0.10.0

### Styling

- [`blob-explorers`] Accomodate new blobscan API changes ([#68](https://github.com/foundry-rs/block-explorers/issues/68))

## [foundry-block-explorers-v0.9.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.9.0) - 2024-11-18

### Dependencies

- Bump compilers ([#67](https://github.com/foundry-rs/block-explorers/issues/67))

### Miscellaneous Tasks

- Release 0.9.0

## [foundry-block-explorers-v0.8.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.8.0) - 2024-09-30

### Miscellaneous Tasks

- Release 0.8.0
- Alloy 0.4 ([#65](https://github.com/foundry-rs/block-explorers/issues/65))

## [foundry-block-explorers-v0.7.3](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.7.3) - 2024-09-19

### Bug Fixes

- Solc_config settings ([#63](https://github.com/foundry-rs/block-explorers/issues/63))

### Miscellaneous Tasks

- Release 0.7.3
- Release 0.7.2

## [foundry-block-explorers-v0.7.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.7.1) - 2024-09-03

### Dependencies

- [deps] Bump compilers ([#62](https://github.com/foundry-rs/block-explorers/issues/62))

### Miscellaneous Tasks

- Release 0.7.1

## [foundry-blob-explorers-v0.7.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-blob-explorers-v0.7.0) - 2024-08-28

### Miscellaneous Tasks

- Release 0.7.0

## [foundry-block-explorers-v0.6.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.6.0) - 2024-08-28

### Dependencies

- Updated alloy-core and alloy dependencies ([#61](https://github.com/foundry-rs/block-explorers/issues/61))

### Miscellaneous Tasks

- Release 0.6.0

## [foundry-block-explorers-v0.5.2](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.5.2) - 2024-08-27

### Bug Fixes

- Fix bugs about the default EVM version in Solc ([#59](https://github.com/foundry-rs/block-explorers/issues/59))

### Miscellaneous Tasks

- Release 0.5.2
- Improve invalid key checks ([#58](https://github.com/foundry-rs/block-explorers/issues/58))

### Testing

- Add invalid api key response test ([#57](https://github.com/foundry-rs/block-explorers/issues/57))

## [foundry-block-explorers-v0.5.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.5.1) - 2024-07-19

### Dependencies

- Bump compilers ([#55](https://github.com/foundry-rs/block-explorers/issues/55))

### Miscellaneous Tasks

- Release 0.5.1

## [foundry-block-explorers-v0.5.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.5.0) - 2024-06-29

### Dependencies

- [deps] Bump compilers 0.9 ([#54](https://github.com/foundry-rs/block-explorers/issues/54))

### Miscellaneous Tasks

- Release 0.5.0
- Fix up manifests
- [meta] Update CODEOWNERS

### Other

- The EVM version returned by Blockscout is "default"  ([#53](https://github.com/foundry-rs/block-explorers/issues/53))
- Create cache directory if needed ([#52](https://github.com/foundry-rs/block-explorers/issues/52))

## [foundry-block-explorers-v0.4.1](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.4.1) - 2024-06-17

### Dependencies

- Bump compilers ([#49](https://github.com/foundry-rs/block-explorers/issues/49))

### Miscellaneous Tasks

- Release 0.4.1
- Use crates alloy ([#50](https://github.com/foundry-rs/block-explorers/issues/50))

## [foundry-block-explorers-v0.4.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.4.0) - 2024-06-11

### Dependencies

- [deps] Bump compilers ([#48](https://github.com/foundry-rs/block-explorers/issues/48))

### Miscellaneous Tasks

- Release 0.4.0
- Sync cliff.toml

## [foundry-block-explorers-v0.3.0](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.3.0) - 2024-06-03

### Dependencies

- Bump compilers ([#47](https://github.com/foundry-rs/block-explorers/issues/47))

### Miscellaneous Tasks

- Release 0.3.0

## [foundry-block-explorers-v0.2.8](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.2.8) - 2024-05-21

### Dependencies

- Bump compilers ([#46](https://github.com/foundry-rs/block-explorers/issues/46))

### Miscellaneous Tasks

- Release 0.2.8

## [foundry-block-explorers-v0.2.7](https://github.com/foundry-rs/block-explorers/releases/tag/vfoundry-block-explorers-v0.2.7) - 2024-05-03

### Bug Fixes

- [blob-explorers] `alloy-serde` usage ([#45](https://github.com/foundry-rs/block-explorers/issues/45))

### Dependencies

- Bump foundry-compilers ([#44](https://github.com/foundry-rs/block-explorers/issues/44))

### Features

- Add blob-explorer crate ([#42](https://github.com/foundry-rs/block-explorers/issues/42))

### Miscellaneous Tasks

- Release 0.2.7
- Convert to workspace ([#41](https://github.com/foundry-rs/block-explorers/issues/41))

## [0.2.6](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.6) - 2024-04-15

### Miscellaneous Tasks

- Release 0.2.6

### Other

- Etherscan cache is constantly invalidated ([#40](https://github.com/foundry-rs/block-explorers/issues/40))

## [0.2.5](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.5) - 2024-04-03

### Dependencies

- Bump alloy-core ([#39](https://github.com/foundry-rs/block-explorers/issues/39))

### Miscellaneous Tasks

- Release 0.2.5

## [0.2.4](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.4) - 2024-03-29

### Bug Fixes

- Avoid setting extension when writing source tree to disk ([#32](https://github.com/foundry-rs/block-explorers/issues/32))

### Dependencies

- [deps] Bump reqwest to 0.12 ([#37](https://github.com/foundry-rs/block-explorers/issues/37))

### Miscellaneous Tasks

- Release 0.2.4
- Remove unused imports ([#33](https://github.com/foundry-rs/block-explorers/issues/33))

### Other

- Chain_id param in etherscan query req ([#36](https://github.com/foundry-rs/block-explorers/issues/36))
- Add concurrency ([#34](https://github.com/foundry-rs/block-explorers/issues/34))

## [0.2.3](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.3) - 2024-01-31

### Dependencies

- Bump foundry-compilers 0.3 ([#30](https://github.com/foundry-rs/block-explorers/issues/30))

### Miscellaneous Tasks

- Release 0.2.3

## [0.2.2](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.2) - 2024-01-26

### Miscellaneous Tasks

- Release 0.2.2
- Include value in serde error ([#29](https://github.com/foundry-rs/block-explorers/issues/29))
- Add unreleased section to cliff.toml
- Fix cliff, update CHANGELOG

## [0.2.1](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.1) - 2024-01-18

### Features

- Add viaIR to VerifyContract ([#28](https://github.com/foundry-rs/block-explorers/issues/28))

### Miscellaneous Tasks

- Release 0.2.1

## [0.2.0](https://github.com/foundry-rs/block-explorers/releases/tag/v0.2.0) - 2024-01-10

### Bug Fixes

- Exclude

### Dependencies

- [deps] Bump compilers
- [deps] Bump alloys ([#27](https://github.com/foundry-rs/block-explorers/issues/27))

### Miscellaneous Tasks

- Release 0.2.0
- Exclude useless directories
- Update cliff link
- Add CHANGELOG.md scripts ([#26](https://github.com/foundry-rs/block-explorers/issues/26))

## [0.1.3](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.3) - 2024-01-05

### Bug Fixes

- Dont force trailing url slash ([#25](https://github.com/foundry-rs/block-explorers/issues/25))
- Fix deserialization error resulting from Blockscout omitting "OptimizationRuns" field when optimization was not used ([#23](https://github.com/foundry-rs/block-explorers/issues/23))
- Fix deserialization failure when fetching contract source_code from blockscout ([#22](https://github.com/foundry-rs/block-explorers/issues/22))

### Miscellaneous Tasks

- Release 0.1.3

### Other

- Add `getcontractcreation` binding ([#24](https://github.com/foundry-rs/block-explorers/issues/24))

## [0.1.2](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.2) - 2023-12-08

### Bug Fixes

- Sanitize all source entries ([#19](https://github.com/foundry-rs/block-explorers/issues/19))

### Miscellaneous Tasks

- 0.1.2 ([#20](https://github.com/foundry-rs/block-explorers/issues/20))

## [0.1.1](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.1) - 2023-11-23

### Dependencies

- Bump Alloy

### Miscellaneous Tasks

- Release 0.1.1
- [meta] Add CODEOWNERS

## [0.1.0](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.0) - 2023-11-15

### Bug Fixes

- Add licensing ([#4](https://github.com/foundry-rs/block-explorers/issues/4))
- [features] Remove ethers-solc for foundry-compilers ([#3](https://github.com/foundry-rs/block-explorers/issues/3))

### Dependencies

- Bump ethers ([#9](https://github.com/foundry-rs/block-explorers/issues/9))

### Documentation

- Add CHANGELOG.md

### Features

- Remove Ethers ([#14](https://github.com/foundry-rs/block-explorers/issues/14))
- Repo improvements ([#11](https://github.com/foundry-rs/block-explorers/issues/11))
- Alloy migration ([#2](https://github.com/foundry-rs/block-explorers/issues/2))
- [`CI`] Enable ci/cd ([#1](https://github.com/foundry-rs/block-explorers/issues/1))
- Repo init

### Miscellaneous Tasks

- [meta] Update configs ([#15](https://github.com/foundry-rs/block-explorers/issues/15))
- Remove RawAbi and LosslessAbi usage ([#12](https://github.com/foundry-rs/block-explorers/issues/12))
- Enable more lints ([#13](https://github.com/foundry-rs/block-explorers/issues/13))
- Remove default feats from openssl ([#7](https://github.com/foundry-rs/block-explorers/issues/7))
- Patch ethers to be in sync w/ foundry ([#6](https://github.com/foundry-rs/block-explorers/issues/6))
- Clippy ([#5](https://github.com/foundry-rs/block-explorers/issues/5))

### Other

- Update README.md

### Styling

- Update rustfmt config ([#16](https://github.com/foundry-rs/block-explorers/issues/16))

<!-- generated by git-cliff -->
