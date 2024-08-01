use crate::models::http_data::HttpResponse;
use scraper::{Html, Selector};

pub struct FormAnalyzer;

#[derive(Debug)]
pub struct FormInfo {
    pub action: String,
    pub method: String,
    pub inputs: Vec<InputInfo>,
}

#[derive(Debug)]
pub struct InputInfo {
    pub name: String,
    pub input_type: String,
}

impl FormAnalyzer {
    pub fn analyze(&self, response: &HttpResponse) -> Vec<FormInfo> {
        let document = Html::parse_document(&response.body);
        let form_selector = Selector::parse("form").unwrap();

        document.select(&form_selector)
            .map(|form_element| {
                let action = form_element.value().attr("action").unwrap_or("").to_string();
                let method = form_element.value().attr("method").unwrap_or("GET").to_string();

                let input_selector = Selector::parse("input").unwrap();
                let inputs = form_element.select(&input_selector)
                    .map(|input| {
                        InputInfo {
                            name: input.value().attr("name").unwrap_or("").to_string(),
                            input_type: input.value().attr("type").unwrap_or("text").to_string(),
                        }
                    })
                    .collect();

                FormInfo { action, method, inputs }
            })
            .collect()
    }
}
