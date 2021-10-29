use from_response::FromResponse;
use http::{request, Request, Response};
use hyper::Body;
use serde::Serialize;
use tame_oauth::{
    gcp::{TokenOrRequest, TokenProvider, TokenProviderWrapper},
    Token,
};
use tower::{buffer::Buffer, util::BoxService, BoxError, Layer, Service, ServiceExt};
use tower_http::map_response_body::MapResponseBodyLayer;
use url::Url;

pub mod api;
mod body;
// Add `into_stream()` to `http::Body`
use body::BodyStreamExt;
mod error;
mod from_response;

pub type Result<T, E = error::DnsError> = std::result::Result<T, E>;

/// The Cloud DNS API client.
#[derive(Clone)]
pub struct DnsClient {
    inner: Buffer<BoxService<Request<Body>, Response<Body>, BoxError>, Request<Body>>,
    pub base_url: url::Url,
    project_id: String,
}

impl DnsClient {
    pub fn new<S, B>(service: S, project_id: &str) -> Self
    where
        S: Service<Request<Body>, Response = Response<B>> + Send + 'static,
        S::Future: Send + 'static,
        S::Error: Into<BoxError>,
        B: http_body::Body<Data = bytes::Bytes> + Send + 'static,
        B::Error: std::error::Error + Send + Sync + 'static,
    {
        let service = MapResponseBodyLayer::new(|b: B| Body::wrap_stream(b.into_stream()))
            .layer(service)
            .map_err(|e| e.into());

        Self {
            inner: Buffer::new(BoxService::new(service), 1024),
            project_id: project_id.to_string(),
            base_url: Url::parse(&base_url(project_id)).unwrap(),
        }
    }

    pub fn changes(&self) -> api::changes::ChangesHandler {
        api::changes::ChangesHandler::new(self)
    }

    pub fn dns_keys(&self) -> api::dns_keys::DnsKeysHandler {
        api::dns_keys::DnsKeysHandler::new(self)
    }

    pub fn managed_zone_operations(
        &self,
    ) -> api::managed_zone_operations::ManagedZoneOperationsHandler {
        api::managed_zone_operations::ManagedZoneOperationsHandler::new(self)
    }

    pub fn managed_zones(&self) -> api::managed_zones::ManagedZonesHandler {
        api::managed_zones::ManagedZonesHandler::new(self)
    }

    pub fn policies(&self) -> api::policies::PoliciesHandler {
        api::policies::PoliciesHandler::new(self)
    }

    pub fn projects(&self) -> api::projects::ProjectsHandler {
        api::projects::ProjectsHandler::new(self)
    }

    pub fn resource_record_sets(&self) -> api::resource_record_sets::ResourceRecordSetsHandler {
        api::resource_record_sets::ResourceRecordSetsHandler::new(self)
    }
}

impl DnsClient {
    pub fn absolute_url(&self, path: impl AsRef<str>) -> Result<url::Url> {
        self.base_url
            .join(path.as_ref())
            .map_err(error::DnsError::Url)
    }

    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        let builder = self
            .request_builder(self.absolute_url(route)?, http::Method::POST)
            .await?;

        let request = match body {
            Some(b) => builder.body(Body::from(
                serde_json::to_string(b).map_err(error::DnsError::JsonTest)?,
            ))?,
            None => builder.body(Body::empty())?,
        };

        let response = self.execute(request).await?;
        R::from_response(response).await
    }

    pub async fn get<R, A>(&self, route: A) -> Result<R>
    where
        A: AsRef<str>,
        R: FromResponse,
    {
        let builder = self
            .request_builder(self.absolute_url(route)?, http::Method::GET)
            .await?;

        let response = self.execute(builder.body(Body::empty())?).await?;
        R::from_response(response).await
    }

    pub async fn patch<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let builder = self
            .request_builder(self.absolute_url(route)?, http::Method::PATCH)
            .await?;

        let request = match body {
            Some(b) => builder.body(Body::from(
                serde_json::to_string(b).map_err(error::DnsError::JsonTest)?,
            ))?,
            None => builder.body(Body::empty())?,
        };

        let response = self.execute(request).await?;
        R::from_response(response).await
    }

    pub async fn put<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let builder = self
            .request_builder(self.absolute_url(route)?, http::Method::PUT)
            .await?;

        let request = match body {
            Some(b) => builder.body(Body::from(
                serde_json::to_string(b).map_err(error::DnsError::JsonTest)?,
            ))?,
            None => builder.body(Body::empty())?,
        };

        let response = self.execute(request).await?;
        R::from_response(response).await
    }

    pub async fn delete<R, A>(&self, route: A) -> Result<R>
    where
        A: AsRef<str>,
        R: FromResponse,
    {
        let builder = self
            .request_builder(self.absolute_url(route)?, http::Method::DELETE)
            .await?;

        let response = self.execute(builder.body(Body::empty())?).await?;
        R::from_response(response).await
    }

    pub async fn request_builder(
        &self,
        url: url::Url,
        method: http::Method,
    ) -> Result<http::request::Builder> {
        match self.fetch_token().await {
            Ok(token) => Ok(request::Builder::new()
                .method(method)
                .uri(url.to_string())
                .header(
                    http::header::AUTHORIZATION,
                    format!("Bearer {}", token.access_token),
                )),
            Err(e) => Err(e),
        }
    }

    pub async fn execute(&self, request: Request<Body>) -> Result<Response<Body>> {
        let mut svc = self.inner.clone();
        svc.ready()
            .await
            .map_err(error::DnsError::Service)?
            .call(request)
            .await
            .map_err(error::DnsError::Service)
    }
}

impl DnsClient {
    async fn fetch_token(&self) -> Result<Token> {
        let provider = TokenProviderWrapper::get_default_provider()
            .expect("unable to read default token provider")
            .expect("unable to find default token provider");

        match provider.get_token(scopes()).unwrap() {
            TokenOrRequest::Request {
                request,
                scope_hash,
                ..
            } => {
                let (parts, body) = request.into_parts();

                let mut request_builder = request::Builder::new();

                for (key, value) in parts.headers.iter() {
                    request_builder
                        .headers_mut()
                        .unwrap()
                        .append(key, value.clone());
                }

                let request = request_builder
                    .method(parts.method)
                    .uri(parts.uri.to_string())
                    .body(Body::from(body))
                    .unwrap();

                let response = self.execute(request).await.unwrap();

                let mut response_builder = http::Response::builder()
                    .status(response.status())
                    .version(response.version());

                let headers = response_builder.headers_mut().unwrap();
                headers.extend(
                    response
                        .headers()
                        .into_iter()
                        .map(|(k, v)| (k.clone(), v.clone())),
                );

                provider
                    .parse_token_response(
                        scope_hash,
                        response_builder
                            .body(hyper::body::to_bytes(response.into_body()).await?)?,
                    )
                    .map_err(error::DnsError::Auth)
            }
            _ => unreachable!(),
        }
    }
}

fn base_url(project_id: &str) -> String {
    format!("https://dns.googleapis.com/dns/v1/projects/{}/", project_id)
}

fn scopes() -> &'static [&'static str] {
    &["https://www.googleapis.com/auth/ndev.clouddns.readwrite"]
}
