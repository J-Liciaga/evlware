/*
HTTP Web Application Security Testing Utility

TODO:
- implement sophisticated cookie handling
- add methods for handling (JSON, XML, form data, ...etc)
- handle different authentication strategies
- add support for HTTP/2 and WebSockets
- improve TLS config options
- add warnings / require explicit configuration for potentially unsafe operations (e.g disabling certificate verification)
*/

use reqwest::{Client, Response, RequestBuilder};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;
use async_trait::async_trait;

// configuration of the client
pub struct HttpClientConfig {
    timeout: Duration,
    user_agent: String,
    proxy: Option<String>,
    follow_redirects: bool,
    max_redirects: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        HttpClientConfig {
            timeout: Duration::from_secs(30),
            user_agent: "netscout web security scanner".to_string(),
            proxy: None,
            follow_redirects: true,
            max_redirects: 10,
        }
    }
}

pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
}

// defines core async methods
#[async_trait]
pub trait AsyncHttpClient {
    async fn get(
        &self,
        url: &str,
    ) -> Result<Response, reqwest::Error>;

    async fn post(
        &self,
        url: &str,
        body: &str,
    ) -> Result<Response, reqwest::Error>;

    async fn request(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&str>
    ) -> Result<Response, reqwest::Error>;
}

impl HttpClient {
    pub fn new(
        config: HttpClientConfig
    ) -> Result<Self, reqwest::Error> {
        let mut client_builder = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(config.max_redirects));

        if let Some(proxy) = &config.proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy)?);
        }

        let client = client_builder.build()?;

        Ok(HttpClient { client, config })
    }

    // allows for consistent preparation of all request
    fn prepare_request(
        &self,
        builder: RequestBuilder
    ) -> RequestBuilder {
        builder.header(USER_AGENT, &self.config.user_agent)
    }
}

#[async_trait]
impl AsyncHttpClient for HttpClient {
    async fn get(
        &self,
        url: &str
    ) -> Result<Response, reqwest::Error> {
        self.prepare_request(self.client.get(url)).send().await
    }

    async fn post(
        &self,
        url: &str,
        body: &str
    ) -> Result<Response, reqwest::Error> {
        let mut builder = self.preapre_request(self.client.request(method, url));

        if let Some(body_content) = body {
            builder = builder.body(body_content.to_string());
        }

        builder.send().await
    }
}

impl HttpClient {
    pub async fn extract_csrf_token(
        &self,
        url: &str
    ) -> Result<Option<String>, reqwest::Error> {
        todo!()
    }

    pub fn set_random_user_agent(
        &mut self
    ) {
        todo!()
    }

    pub async fn fuzz_headers(
        &self,
        url: &str,
        headers: HeaderMap
    ) -> Result<Vec<Response>, reqwest::Error> {
        todo!()
    }
}