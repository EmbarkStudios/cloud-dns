use from_response::FromResponse;
use reqwest::{Url};
use serde::Serialize;
use tame_oauth::{Token, gcp::{TokenOrRequest, TokenProvider, TokenProviderWrapper}};

mod api;
mod error;
mod from_response;

pub type Result<T, E = error::DNSError> = std::result::Result<T, E>;

/// The Cloud DNS API client.
#[derive(Debug, Clone)]
pub struct DNSClient {
    client: reqwest::Client,
    pub base_url: Url,
    project_id: String,
}

impl DNSClient {
    pub fn new(project_id: String) -> Result<DNSClient> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(error::DNSError::Http)?;

        Ok(DNSClient {
            client,
            project_id: project_id.clone(),
            base_url: Url::parse(&base_url(project_id)).unwrap(),
        })
    }

    pub fn changes(&self) -> api::changes::ChangesHandler {
        api::changes::ChangesHandler::new(self)
    }

    pub fn dns_keys(&self) -> api::dns_keys::DNSKeysHandler {
        api::dns_keys::DNSKeysHandler::new(self)
    }

    pub fn managed_zone_operations(&self) {}

    pub fn managed_zones(&self) {}

    pub fn policies(&self) {}

    pub fn projects(&self) {}

    pub fn resource_record_sets(&self) -> api::resource_record_sets::ResourceRecordSetsHandler {
        api::resource_record_sets::ResourceRecordSetsHandler::new(self)
    }
}

impl DNSClient {
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        self.base_url
            .join(url.as_ref())
            .map_err(error::DNSError::Url)
    }

    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        let mut request = self.request_builder(self.absolute_url(route)?, reqwest::Method::POST).await?;

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = self.execute(request).await?;
        R::from_response(map_dns_error(response).await?).await
    }

    pub async fn get<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let mut request = self.request_builder(self.absolute_url(route)?, reqwest::Method::GET).await?;

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        let response = self.execute(request).await?;
        println!("{:#?}", response);
        R::from_response(map_dns_error(response).await?).await
    }

    pub async fn patch<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let mut request = self.request_builder(self.absolute_url(route)?, reqwest::Method::PATCH).await?;

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = self.execute(request).await?;
        R::from_response(map_dns_error(response).await?).await
    }

    pub async fn put<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let mut request = self.request_builder(self.absolute_url(route)?, reqwest::Method::PUT).await?;

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = self.execute(request).await?;
        R::from_response(map_dns_error(response).await?).await
    }

    pub async fn delete<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let mut request = self.request_builder(self.absolute_url(route)?, reqwest::Method::DELETE).await?;

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        let response = self.execute(request).await?;
        R::from_response(map_dns_error(response).await?).await
    }

    pub async fn request_builder(
        &self,
        url: impl reqwest::IntoUrl,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder> {
        match self.fetch_token().await {
            Ok(token) => { Ok(self.client.request(method, url).bearer_auth(token.access_token)) },
            Err(e) => {Err(e)},
        }
    }

    pub async fn execute(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        request.send().await.map_err(error::DNSError::Http)
    }
}

impl DNSClient {
    async fn fetch_token(&self) -> Result<Token> {
        let provider = TokenProviderWrapper::get_default_provider()
        .expect("unable to read default token provider")
        .expect("unable to find default token provider");

        match provider.get_token(&scopes()).unwrap() {
            TokenOrRequest::Request {
                request,
                scope_hash,
                ..
            } => {
                let client = reqwest::Client::new();
    
                let (parts, body) = request.into_parts();
                let uri = parts.uri.to_string();
    
                let builder = match parts.method {
                    http::Method::POST => client.post(&uri),
                    method => unimplemented!("{} not implemented", method),
                };
    
                let request = builder.headers(parts.headers).body(body).build().unwrap();
                let response = client.execute(request).await.unwrap();
    
                let mut builder = http::Response::builder()
                    .status(response.status())
                    .version(response.version());
    
                let headers = builder.headers_mut().unwrap();
                headers.extend(
                    response
                        .headers()
                        .into_iter()
                        .map(|(k, v)| (k.clone(), v.clone())),
                );
    
                let buffer = response.bytes().await.unwrap();
                let response = builder.body(buffer).unwrap();
    
                provider.parse_token_response(scope_hash, response).map_err(error::DNSError::Auth)
            },
            _ => unreachable!(),
        }
    }
}

pub async fn map_dns_error(response: reqwest::Response) -> Result<reqwest::Response> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(error::DNSError::Http(response.error_for_status().unwrap_err()))
    }
}

fn base_url(project_id: String) -> String {
    format!("https://dns.googleapis.com/dns/v1/projects/{}/", project_id)
}

fn scopes() -> Vec<String> {
    vec!["https://www.googleapis.com/auth/ndev.clouddns.readwrite".to_string()]
}
