use tokio::net::TcpStream;
use tokio::time::Duration;

pub async fn send_tcp_syn(
    host: &str, 
    port: u16, 
    timeout: Duration
) -> Result<Duration, std::io::Error> {
    let start = std::time::Instant::now();
    let addr = format!("{}:{}", host, port);
    let result = tokio::time::timeout(timeout, TcpStream::connect(&addr)).await;

    match result {
        Ok(Ok(_)) => Ok(start.elapsed()),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timed out"))
    }
}
