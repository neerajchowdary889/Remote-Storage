use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_server() -> Result<(), Box<dyn Error>> {
        let mut client = TcpStream::connect("127.0.0.1:5000").await?;
        let message = "Hello, server!";
        client.write_all(message.as_bytes()).await?;

        let mut buffer = [0; 1024];
        let size = client.read(&mut buffer).await?;
        let received_data = String::from_utf8_lossy(&buffer[..size]);

        assert_eq!(received_data, "Message received by the server");

        Ok(())
    }
}