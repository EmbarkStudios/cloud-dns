#[tokio::main]
async fn main() -> cloud_dns::Result<()> {
    let project_id = std::env::var("PROJECT_ID").expect("PROJECT_ID env variable is required");
    let managed_zone =
        std::env::var("MANAGED_ZONE").expect("MANAGED_ZONE env variable is required");

    let response = cloud_dns::DNSClient::new(project_id)?
        .changes()
        .list(managed_zone)
        .await?;

    println!("{:#?}", response);

    Ok(())
}
