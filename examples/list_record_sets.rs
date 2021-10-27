use std::convert::TryFrom;

#[tokio::main]
async fn main() -> cloud_dns::Result<()> {
    let project_id = std::env::var("PROJECT_ID").expect("PROJECT_ID env variable is required");
    let managed_zone =
        std::env::var("MANAGED_ZONE").expect("MANAGED_ZONE env variable is required");

    let client = reqwest::Client::new();

    let mut svc = tower::ServiceBuilder::new()
        .service(tower::service_fn(|req: http::Request<hyper::body::Body>| async move {
            let response = client.execute(
                reqwest::Request::try_from(req).unwrap()
            ).await.unwrap();

            let mut builder = http::Response::builder()
                .status(response.status())
                .version(response.version());

            let headers = builder.headers_mut().unwrap();

            // Unfortunately http doesn't expose a way to just use
            // an existing HeaderMap, so we have to copy them :(
            headers.extend(
                response
                    .headers()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );

            builder.body(response.bytes()).unwrap()
        }
    ));

    let response = cloud_dns::DnsClient::new(svc, project_id.as_str())
        .changes()
        .list(managed_zone.as_str())
        .await?;

    println!("{:#?}", response);

    Ok(())
}
