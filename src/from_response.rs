/// A trait for mapping from a `reqwest::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> super::Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> super::Result<Self> {
        let text = response.text().await?;

        let de = &mut serde_json::Deserializer::from_str(&text);
        serde_path_to_error::deserialize(de).map_err(super::error::DNSError::Json)
    }
}
