use crate::ProtoServer;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

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
impl ProtoServer for PrimeTime {
    async fn run_server(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        let mut buf = [0; 1024];
        loop {
            // TODO: read until new line/ASCII 10?
            let bytes = self.read(&mut socket, &mut buf).await?;

            let req: Request = match serde_json::from_slice(bytes) {
                Ok(v) => v,
                Err(_) => return self.write(&mut socket, bytes).await,
            };

            let prime = is_prime_float(req.number);

            let response = Response {
                method: Method::IsPrime,
                prime,
            };

            let output_bytes = serde_json::to_vec(&response)?;
            self.write(&mut socket, &output_bytes).await?;
            // Responses are always newline-terminated
            self.write(&mut socket, "\n".as_bytes()).await?;
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
