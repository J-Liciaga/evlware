mod subdomain_enumerator;
mod directory_enumerator;
mod api_enumerator;
mod tech_enumerator;

pub use subdomain_enumerator::SubdomainEnumerator;
pub use directory_enumerator::DirectoryEnumerator;
pub use api_enumerator::ApiEnumerator;
pub use tech_enumerator::TechEnumerator;

use crate::models::Target;
use crate::utils::error::Error;

pub struct EnumerationResult {
    subdomains: Vec<String>,
    directories: Vec<String>,
    api_endpoints: Vec<String>,
    technologies: Vec<String>,
}

pub trait Enumerator {
    fn enumerate(&self, target: &Target) -> Result<EnumerationResult, Error>;
}
