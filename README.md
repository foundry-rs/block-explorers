# foundry-block-explorers

Crates for interacting with explorer APIS.

[![Build Status][actions-badge]][actions-url]
[![Telegram chat][telegram-badge]][telegram-url]

[actions-badge]: https://img.shields.io/github/actions/workflow/status/foundry-rs/block-explorers/ci.yml?branch=main&style=for-the-badge
[actions-url]: https://github.com/foundry-rs/block-explorers/actions?query=branch%3Amain
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[telegram-url]: https://t.me/foundry_rs

## Overview

This repository contains the following crates:

- [`foundry-block-explorers`]: Bindings for etherscan.io API.
- [`foundry-blob-explorers`]: Bindings for blobscan.com API.


## Supported Rust Versions

<!--
When updating this, also update:
- clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

We will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.65.0.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## Contributing

Thanks for your help improving the project! We are so happy to have you! We have
[a contributing guide](./CONTRIBUTING.md) to help you get involved in the
Alloy project.

Pull requests will not be merged unless CI passes, so please ensure that your
contribution follows the linting rules and passes clippy.


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>