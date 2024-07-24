use url::Url;
use crate::models::error::{Error, InvalidTargetError};

fn validate_url(url_str: &str) -> Result<Url, Error> {
    match Url::parse(url_str) {
        Ok(url) => Ok(url),
        Err(e) => Err(Error::InvalidTarget(InvalidTargetError::MalformedURL(
            format!("Failed to parse URL: {}", e)
        )))
    }
}
