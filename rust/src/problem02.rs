use std::collections::BTreeMap;

use crate::{write_util, ProtoServer};
use async_trait::async_trait;

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

#[derive(Debug, Default)]

struct PriceData(BTreeMap<Timestamp, Price>);

impl PriceData {
    fn insert(&mut self, time: Timestamp, price: Price) -> Option<Price> {
        self.0.insert(time, price)
    }

    fn mean(&self, min: Timestamp, max: Timestamp) -> anyhow::Result<i32> {
        use core::ops::Bound::Included;
        if min > max {
            return Ok(0);
        }

        let (count, sum): (i64, i64) = self
            .0
            .range(((Included(min)), Included(max)))
            .fold((0, 0), |(count, sum), (_, val)| {
                (count + 1, sum + (val.0 as i64))
            });

        if count == 0 {
            return Ok(0);
        }
        Ok((sum / count).try_into()?)
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

        let mut session_data = PriceData::default();
        loop {
            let mut buf = [0; 9];
            let data_read = buf_reader.read_exact(&mut buf).await?;

            if data_read == 0 {
                // done with client
                return Ok(());
            }

            match parse_message(&buf)? {
                Message::Insert(time, price) => {
                    session_data.insert(time, price);
                }
                Message::Query { min, max } => {
                    let average = session_data.mean(min, max)?;
                    write_util(&mut writer, &average.to_be_bytes()).await?;
                }
            }
        }
    }
}
