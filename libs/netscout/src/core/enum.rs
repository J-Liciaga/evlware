use crate::models::Target;
use crate::modules::enumeration::{
    Enumerator, EnumerationResult, SubdomainEnumerator, DirectoryEnumerator, ApiEnumerator, TechEnumerator
};
use crate::utils::error::Error;
use crate::utils::http_client::HttpClient;

pub struct EnumerationManager {
    enumerators: Vec<Box<dyn Enumerator>>,
}

impl EnumerationManager {
    pub fn new(http_client: HttpClient) -> Self {
        let enumerators: Vec<Box<dyn Enumerator>> = vec![
            Box::new(SubdomainEnumerator::new(http_client.clone())),
            Box::new(DirectoryEnumerator::new(http_client.clone())),
            Box::new(ApiEnumerator::new(http_client.clone())),
            Box::new(TechEnumerator::new(http_client)),
        ];

        Self { enumerators }
    }

    pub async fn enumerate(&self, target: &Target) -> Result<EnumerationResult, Error> {
        let mut final_result = EnumerationResult {
            subdomains: vec![],
            directories: vec![],
            api_endpoints: vec![],
            technologies: vec![],
        };

        for enumerator in &self.enumerators {
            let result = enumerator.enumerate(target)?;
            final_result.subdomains.extend(result.subdomains);
            final_result.directories.extend(result.directories);
            final_result.api_endpoints.extend(result.api_endpoints);
            final_result.technologies.extend(result.technologies);
        }

        Ok(final_result)
    }
}
