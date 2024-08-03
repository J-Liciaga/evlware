use crate::models::http_data::HttpResponse;
use url::Url;
use std::time::Duration;
use std::collections::HashMap;
use reqwest::Client; 

pub async fn send_http_request(
    url: &Url, 
    timeout: Duration
) -> Result<HttpResponse, reqwest::Error> {
    let client = Client::builder()
        .timeout(timeout)
        .build()?;

    let response = client
        .get(url.as_str())
        .send()
        .await?;

    let status_code = response
        .status()
        .as_u16();

    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| (
                name
                    .as_str()
                    .to_owned(),
                value
                    .to_str()
                    .unwrap_or("")
                    .to_owned()
            ),
        )
        .collect::<HashMap<String, String>>();

    let body = response
        .text()
        .await?;

    Ok(HttpResponse::new(
        status_code,
        headers,
        body,
    ))
}
