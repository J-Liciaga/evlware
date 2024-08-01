use super::results::ScanResults;

#[derive(Debug)]
pub struct Report {
    pub scan_results: Vec<ScanResults>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub summary: String,
}

impl Report {

}
