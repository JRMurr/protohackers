mod problems;

use crate::problems::run_problem;
use anyhow::{anyhow, Context};
use clap::Parser;
use env_logger::{Env, Target};
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// A lot of this shamelessly stolen from https://github.com/LucasPickering/protohackers

/// TCP server for Protohackers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of the problem whose server should be executed
    #[clap(value_parser, env)]
    problem: u8,

    /// IP/hostname to bind to
    #[clap(long, value_parser, default_value = "0.0.0.0")]
    host: String,

    /// Port number to host on
    #[clap(short, long, value_parser, default_value_t = 8050)]
    port: u16,
}

async fn read_util<'a, T: AsyncReadExt + std::marker::Unpin>(
    socket: &mut T,
    buffer: &'a mut [u8],
) -> anyhow::Result<&'a [u8]> {
    let bytes_read = socket
        .read(buffer)
        .await
        .context("Error reading from socket")?;
    if bytes_read == 0 {
        Err(anyhow!("Socket closed"))
    } else {
        Ok(&buffer[0..bytes_read])
    }
}

async fn write_util<T: AsyncWriteExt + std::marker::Unpin>(
    socket: &mut T,
    bytes: &[u8],
) -> anyhow::Result<()> {
    socket
        .write_all(bytes)
        .await
        .context("Error writing to socket")
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .target(Target::Stdout)
        .init();
    let args = Args::parse();
    let listener = TcpListener::bind((args.host.as_str(), args.port)).await?;
    info!("Listening on {}:{}", args.host, args.port);
    run_problem(listener, args.problem).await?;
    Ok(())
}
