use crate::{read_util, write_util, ProtoServer};
use async_trait::async_trait;
use tokio::net::TcpStream;

/// Problem 0 - echo back input
#[derive(Copy, Clone, Debug)]
pub struct EchoServer;

#[async_trait]
impl ProtoServer for EchoServer {
    async fn run_server(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        let mut buf = [0; 1024];
        // Read until the client closes the socket
        loop {
            let bytes = read_util(&mut socket, &mut buf).await?;
            write_util(&mut socket, bytes).await?;
        }
    }
}
