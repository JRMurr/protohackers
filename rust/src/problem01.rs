use crate::{write_util, StatelessServer};
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};

#[derive(Debug, Serialize, Deserialize)]
enum Method {
    #[serde(rename = "isPrime")]
    IsPrime,
}

/// Problem 1 - PrimeTime
#[derive(Copy, Clone, Debug)]
pub struct PrimeTime;

#[derive(Debug, Deserialize)]
struct Request {
    #[allow(dead_code)]
    method: Method,
    number: f64,
}

#[derive(Debug, Serialize)]
struct Response {
    method: Method,
    prime: bool,
}

#[async_trait]
impl StatelessServer for PrimeTime {
    async fn run_server(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        let (reader, mut writer) = socket.split();

        let mut buf_reader = BufReader::new(reader);
        loop {
            // let bytes = read_util(&mut socket, &mut buf).await?;
            let mut buf = String::new();
            let data_read = buf_reader.read_line(&mut buf).await?;

            if data_read == 0 {
                // done with client
                return Ok(());
            }

            let req: Request = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(_) => {
                    info!("invalid bytes: {:?}", buf);
                    return write_util(&mut socket, buf.as_bytes()).await;
                }
            };

            let prime = is_prime_float(req.number);

            let response = Response {
                method: Method::IsPrime,
                prime,
            };

            let output_bytes = serde_json::to_vec(&response)?;
            write_util(&mut writer, &output_bytes).await?;
            // Responses are always newline-terminated
            write_util(&mut writer, "\n".as_bytes()).await?;
        }
    }
}

fn is_prime_float(num: f64) -> bool {
    // 1 is not prime i hope
    if num <= 1.0 {
        return false;
    }

    let truncated = num.trunc();
    if num != truncated {
        // not an integer
        return false;
    }

    // TODO: maybe return result if num not in range?
    let int = (truncated as usize).try_into().unwrap();

    is_prime(int)
}

fn is_prime(n: u64) -> bool {
    // https://en.wikipedia.org/wiki/Primality_test#C,_C++,_C#_&_D
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let max_test_num = (n as f32).sqrt() as u64;
    for i in (5..max_test_num).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}
