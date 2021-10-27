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
