use std::collections::HashSet;
use async_trait::asyn_trait;
use futures::future::join_all;

#[async_trait]
trait EnumerationModule {
    async fn evl_enum(
        &self,
        target: &str,
    ) -> Result<HashSet<String>, Box<dyn std::error::Error>>
}

#[derive(Default)]
struct EnumerationResult {
    subdomains: HashSet<String>,
    open_ports: HashSet<String>,
    directories: HashSet<String>,
    api_endpoints: HashSet<String>,
}

struct EvlEnumerator {
    modules: Vec<Box<dyn EnumerationModule>>,
}

impl EvlEnumerator {
    fn new() -> Self {
        EvlEnumerator {
            modules: vec![
                Box::new(subdomain_enum::SubdomainEnumerator::new()),
                Box::new(port_scan::PortScanner::new()),
                Box::new(directory_bruteforce::DirectoryBruteForce::new()),
                Box::new(api_endpoint_discovery::ApiEndpointDiscovery::new()),
            ],
        }
    }

    async fn enumerate(
        &self,
        target: &str,
    ) -> Result<EnumerationResult, Box<dyn std::error::Error>> {
        let futures = self.modules
            .iter()
            .map(|module| module.enumerate(target));

        let results = join_all(futures).await;

        let mut enumeration_result = EnumerationResult::default();

        for (i, result) in results.into_iter().enumerate() {
            match (i, result) {
                (0, Ok(subdomains)) => enumeration_result.subdomains = subdomains,
                (1, Ok(ports)) => enumeration_result.open_ports = ports,
                (2, Ok(directories)) => enumeration_result.directories = directories
            }
        }
    }
}
