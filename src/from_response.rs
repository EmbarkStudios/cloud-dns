use hyper::Body;

/// A trait for mapping from a `reqwest::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: http::Response<Body>) -> super::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: http::Response<Body>) -> super::Result<Self> {
        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let text = String::from_utf8(body_bytes.to_vec())?;

        let de = &mut serde_json::Deserializer::from_str(&text);
        serde_path_to_error::deserialize(de).map_err(super::error::DnsError::Json)
    }
}
