<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `📒 cloud-dns`

**`cloud-dns` is a crate providing a client to interact with [Google Cloud DNS v1](https://cloud.google.com/dns)**
    
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/rust-gpu.svg)](https://crates.io/crates/cloud-dns)
[![Docs](https://docs.rs/cloud-dns/badge.svg)](https://docs.rs/cloud-dns)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/cloud-dns/status.svg)](https://deps.rs/repo/github/EmbarkStudios/cloud-dns)
[![Build status](https://github.com/EmbarkStudios/cloud-dns/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/cloud-dns/actions)
</div>

## Example

```rust
#[tokio::main]
async fn main() -> cloud_dns::Result<()> {
    let project_id = std::env::var("PROJECT_ID").expect("PROJECT_ID env variable is required");
    let managed_zone =
        std::env::var("MANAGED_ZONE").expect("MANAGED_ZONE env variable is required");

    let service = tower::ServiceBuilder::new()
        .service(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()));

    let response = cloud_dns::DnsClient::new(service, project_id.as_str())
        .changes()
        .list(managed_zone.as_str())
        .await?;

    println!("{:#?}", response);

    Ok(())
}
```

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](../main/CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
