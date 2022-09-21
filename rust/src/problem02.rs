use crate::{write_util, ProtoServer};
use async_trait::async_trait;

use rusqlite::Connection;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Timestamp(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Price(i32);

#[derive(Debug)]
enum Message {
    Insert(Timestamp, Price),
    Query { min: Timestamp, max: Timestamp },
}

fn parse_nums(nums: &[u8]) -> anyhow::Result<(i32, i32)> {
    let (first_num, second_num) = nums.split_at(std::mem::size_of::<i32>());
    Ok((
        i32::from_be_bytes(first_num.try_into()?),
        i32::from_be_bytes(second_num.try_into()?),
    ))
}

fn parse_message(input: &[u8]) -> anyhow::Result<Message> {
    if input.len() != 9 {
        anyhow::bail!("Invalid message length")
    }

    let (first, second) = parse_nums(&input[1..])?;

    Ok(match input[0] {
        b'I' => Message::Insert(Timestamp(first), Price(second)),
        b'Q' => Message::Query {
            min: Timestamp(first),
            max: Timestamp(second),
        },
        _ => anyhow::bail!("Invalid message type"),
    })
}

#[derive(Debug)]

struct PriceData {
    conn: Connection,
}

impl PriceData {
    fn new() -> anyhow::Result<Self> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE prices (
                time   INTEGER,
                price  INTEGER
            )",
            (),
        )?;

        Ok(Self { conn })
    }

    fn insert(
        &mut self,
        time: Timestamp,
        price: Price,
    ) -> anyhow::Result<usize> {
        Ok(self.conn.execute(
            "INSERT INTO prices (time, price) VALUES (?1, ?2)",
            (&time.0, &price.0),
        )?)
    }

    fn mean(&self, min: Timestamp, max: Timestamp) -> anyhow::Result<i32> {
        let mut stmt = self.conn.prepare_cached(
            "
            SELECT avg(price)
            FROM prices
            WHERE
                time >= :min AND time <= :max
        ",
        )?;

        Ok(
            stmt.query_row(&[(":min", &min.0), (":max", &max.0)], |row| {
                Ok(match row.get_ref(0)? {
                    rusqlite::types::ValueRef::Null => 0,
                    rusqlite::types::ValueRef::Real(avg) => avg.round() as i32,
                    _ => 0,
                })
            })?,
        )
    }
}

/// Problem 2 - Means to an End
#[derive(Copy, Clone, Debug)]
pub struct MeansToAnEnd;

#[async_trait]
impl ProtoServer for MeansToAnEnd {
    async fn run_server(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        let (reader, mut writer) = socket.split();

        let mut buf_reader = BufReader::new(reader);

        let mut session_data = PriceData::new()?;
        loop {
            let mut buf = [0; 9];
            let data_read = buf_reader.read_exact(&mut buf).await?;

            if data_read == 0 {
                // done with client
                return Ok(());
            }

            match parse_message(&buf)? {
                Message::Insert(time, price) => {
                    session_data.insert(time, price)?;
                }
                Message::Query { min, max } => {
                    let average = session_data.mean(min, max)?;
                    write_util(&mut writer, &average.to_be_bytes()).await?;
                }
            }
        }
    }
}
