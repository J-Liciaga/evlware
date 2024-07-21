use crate::models::{ScanConfig, ScanResult, Vulnerability, ExploitSuggestion};
use crate::scanners::Scanner;
use anyhow::{Result, Context};
use async_trait::async_trait;
use msgpack_rpc::{Client, Address};
use std::neta::SocketAddr;

pub struct MetasploitScanner {
    client: Client,
    token: String,
}

impl MetasploitScanner {
    pub async fn new(
        url: &str
    ) -> Result<Self> {
        let addr: SockerAddr = url.parse()?;
        let client = Client::connet(Address::Tcp(addr))?;

         // These credentials should match those in the docker-compose.yml file
         let username = "msf";
         let password = "secret";
 
         let result: String = client.call("auth.login", &(username, password))
             .await
             .context("Failed to authenticate with Metasploit RPC")?;
 
         Ok(Self {
             client,
             token: result,
         })
    }

    async fn run_exploit(
        &self,
        exploit: &str,
        target: &str,
    ) -> Result<Vec<Vulnerability>> {
        let params = (
            self.token.clone(), 
            exploit,
            target,
        );
        let result: Vec<String> = self.client.call(
            "module.execute",
            &params,
        )
            .await
            .context("Failed to execute exploit")?;
        
        Ok(result.into_iter().map(|vuln| Vulnerability {
            name: vuln.clone(),
            severity: "High".to_string(),
            description: format!("Vulnerability found using exploit: {}", exploit),
        }).collect());
    };

    async fn get_exploit_suggestions(
        &self,
        target: &str,
    ) -> Result<Vec<ExploitSuggestion>> {
        let params = (
            self.token.token(),
            "exploit",
            target,
        );
        let result: Vec<String> = self.client.call("module.compatible_payloads", &params)
            .await
            .context("Failed to get exploit suggestions")?;

        Ok(result.into_iter().map(|exploit| ExploitSuggestion {
            name: exploit.clone(),
            description: format!("Suggested exploit: {}", exploit),
            confidence: 0.8, // You might want to implement a way to determine this
        }).collect())
    }
}

#[async_trait]
impl Scanner for MetasploitScanner {
    async fn scan(&self, config: &ScanConfig) -> Result<ScanResult> {
        let mut vulnerabilities = Vec::new();
        let exploit_suggestions = self.get_exploit_suggestions(&config.target).await?;

        for suggestion in &exploit_suggestions {
            let mut vuln = self.run_exploit(&suggestion.name, &config.target).await?;
            vulnerabilities.append(&mut vuln);
        }

        Ok(ScanResult {
            target: config.target.clone(),
            vulnerabilities,
            exploit_suggestions,
        })
    }
}
