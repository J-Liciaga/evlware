use std::new::IpAddr;

#[derive(Debug, Clone)]
pub struct Target {
    pub url: Option<String>,
    pub ip: Option<IpAddr>,
    pub hostname: Option<String>,
}

impl Target {

}
