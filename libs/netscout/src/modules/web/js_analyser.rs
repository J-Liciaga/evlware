use crate::models::http_data::HttpResponse;
use regex::Regex;

pub struct JsAnalyzer;

#[derive(Debug)]
pub struct JsInfo {
    pub external_scripts: Vec<String>,
    pub inline_scripts: Vec<String>,
    pub potential_vulnerabilities: Vec<String>,
}

impl JsAnalyzer {
    pub fn analyze(&self, response: &HttpResponse) -> JsInfo {
        let document = &response.body;

        JsInfo {
            external_scripts: self.extract_external_scripts(document),
            inline_scripts: self.extract_inline_scripts(document),
            potential_vulnerabilities: self.identify_potential_vulnerabilities(document),
        }
    }

    fn extract_external_scripts(&self, document: &str) -> Vec<String> {
        let re = Regex::new(r#"<script[^>]+src=["']([^"']+)["'][^>]*>"#).unwrap();
        re.captures_iter(document)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    fn extract_inline_scripts(&self, document: &str) -> Vec<String> {
        let re = Regex::new(r#"<script[^>]*>([\s\S]*?)</script>"#).unwrap();
        re.captures_iter(document)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    fn identify_potential_vulnerabilities(&self, document: &str) -> Vec<String> {
        let patterns = vec![
            (r"eval\s*\(", "Use of eval()"),
            (r"document\.write\s*\(", "Use of document.write()"),
            (r"innerHTML\s*=", "Direct assignment to innerHTML"),
        ];

        patterns.iter()
            .filter_map(|(pattern, description)| {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(document) {
                    Some(description.to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}
