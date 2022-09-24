mod problem00;
mod problem01;
mod problem02;
mod problem03;

use anyhow::anyhow;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

use crate::problems::{
    problem00::EchoServer, problem01::PrimeTime, problem02::MeansToAnEnd,
    problem03::BudgetChat,
};

#[enum_dispatch(StatelessServer)]
enum StatlessServers {
    EchoServer,
    PrimeTime,
    MeansToAnEnd,
    BudgetChat,
}

fn get_server(problem: u8) -> anyhow::Result<StatlessServers> {
    match problem {
        0 => Ok(EchoServer.into()),
        1 => Ok(PrimeTime.into()),
        2 => Ok(MeansToAnEnd.into()),
        3 => Ok(BudgetChat.into()),
        prob => Err(anyhow!("Unknown problem: {}", prob)),
    }
}

/// A server whos clients do not need to share state with each other
#[async_trait]
#[enum_dispatch]
pub trait StatelessServer: Send + Sync {
    async fn run_server(&self, socket: TcpStream) -> anyhow::Result<()>;
}

pub async fn run_problem(
    listener: TcpListener,
    problem: u8,
) -> anyhow::Result<()> {
    loop {
        let (socket, client) = listener.accept().await?;
        let server = get_server(problem)?;
        info!("{} Connected", client);

        tokio::spawn(async move {
            if let Err(e) = server.run_server(socket).await {
                if e.to_string() != "Socket closed" {
                    error!("{} Error running server: {}", client, e);
                }
            }
            info!("{} Disconnected", client);
        });
    }
}
