use reqwest::{Client, Response, header};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;
use anyhow::{Result, anyhow};

pub struct HttpClient {
    client: Client,
}

/**
 * 1. accepts invalid SSL certs (use with caution)
 * 2. provides GET and POST requests custom headers
 * 3. includes a method to handle redirects manually
 * 4. sets a default user-agent that identifies EVLWARE
 * 
 * TODO:
 * 1. handle PUT, DELETE, ...etc.
 * 2. implement rate limiting to avoid overwhelming the server
 */

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .danger_accept_invalid_certs(true) // we need to be cautious with this in prod
            .build()?;

        Ok(Self {
            client,
        })
    }

    pub async fn get(
        &self,
        url: &str,
        headers: Option<HeaderMap>
    ) -> Result<Response> {
        let mut request = self.client.get(url);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request.send().await?;

        Ok(response)
    }

    pub async fn post(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let mut response = self.client
            .post(url)
            .body(body.to_string());

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request.send().await?;

        Ok(response)
    }

    pub fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        headers
            .insert(
                header::USER_AGENT,
                HeaderValue::from_static("EVLWARE/1.0")
            );
        headers
            .insert(
                header::ACCEPT,
                HeaderValue::from_static("*/*"),
            );
        
        headers
    }

    pub async fn send_with_redirect(
        &self,
        url: &str,
        max_redirects: usize,
    ) -> Result<Response> {
        let mut response = self.get(url, None).await>?;
        let mut redirect_count = 0;

        while response.status().is_redirection() && redirect_count < max_redirects {
            if let Some(location) = response.headers().get(header::LOCATION) {
                let new_url = location.to_str()?;

                response = self.get(new_url, None).await?;

                redirect_count += 1;
            } else {
                return Err(anyhow!("Redirect location not found"));
            }
        }

        if redirect_count == max_redirects {
            return Err(anyhow!("Max redirects reached"));
        }

        Ok(response)
    }

    pub async fn check_ssl(
        &self,
        url: &str,
    ) -> Result<bool> {
        let response = self.get(url, None).await?;

        Ok(response.url().scheme() == "https")
    }
}
