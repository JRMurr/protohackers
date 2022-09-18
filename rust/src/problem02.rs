use crate::{read_util, write_util, ProtoServer};
use async_trait::async_trait;
use tokio::net::TcpStream;

/// Problem 2 - Means to an End
#[derive(Copy, Clone, Debug)]
pub struct MeansToAnEnd;

#[async_trait]
impl ProtoServer for MeansToAnEnd {
    async fn run_server(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        let mut buf = [0; 1024];
        loop {
            let bytes = read_util(&mut socket, &mut buf).await?;
            write_util(&mut socket, bytes).await?;
        }
    }
}
