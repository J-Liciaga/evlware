use openssl::ssl::{SslMethod, SslConnector};
use openssl::x509::X509;
use std::net::TcpStream;
use url::Url;

pub struct SslChecker;

#[derive(Debug)]
pub struct SslInfo {
    pub version: String,
    pub cipher: String,
    pub certificate_info: CertificateInfo,
}

#[derive(Debug)]
pub struct CertificateInfo {
    pub subject: String,
    pub issuer: String,
    pub version: i32,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
}

impl SslChecker {
    pub fn check(&self, url: &str) -> Result<SslInfo, Box<dyn std::error::Error>> {
        let url = Url::parse(url)?;
        let host = url.host_str().ok_or("Invalid host")?;
        let port = url.port().unwrap_or(443);

        let connector = SslConnector::builder(SslMethod::tls())?.build();
        let stream = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut ssl_stream = connector.connect(host, stream)?;

        let ssl = ssl_stream.ssl();
        let cert = ssl.peer_certificate().ok_or("No peer certificate")?;

        Ok(SslInfo {
            version: ssl.version_str().to_string(),
            cipher: ssl.current_cipher().unwrap().name().to_string(),
            certificate_info: self.extract_certificate_info(&cert),
        })
    }

    fn extract_certificate_info(&self, cert: &X509) -> CertificateInfo {
        CertificateInfo {
            subject: cert.subject_name().entries().next().unwrap().data().as_utf8().unwrap().to_string(),
            issuer: cert.issuer_name().entries().next().unwrap().data().as_utf8().unwrap().to_string(),
            version: cert.version(),
            serial_number: cert.serial_number().to_bn().unwrap().to_dec_str().unwrap(),
            not_before: cert.not_before().to_string(),
            not_after: cert.not_after().to_string(),
        }
    }
}
