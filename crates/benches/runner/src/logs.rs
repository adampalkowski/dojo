use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::KatanaRunner;

#[derive(Serialize, Deserialize)]
struct TimedLog<T> {
    timestamp: String,
    level: String,
    fields: T,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
    target: String,
}

type Log = TimedLog<Message>;

impl KatanaRunner {
    pub fn blocks(&self) -> Vec<String> {
        BufReader::new(File::open(&self.log_filename).unwrap())
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| match serde_json::from_str(&line) {
                Ok(Log { fields: Message { message, .. }, .. }) => Some(message),
                Err(_) => None,
            })
            .filter_map(|message| match message.contains("⛏️ Block") {
                true => Some(message),
                false => None,
            })
            .collect()
    }

    pub async fn blocks_until_empty(&self) -> Vec<String> {
        let mut blocks = self.blocks();
        loop {
            if let Some(block) = blocks.last() {
                if block.contains("mined with 0 transactions") {
                    break;
                }
            }

            let len_at_call = blocks.len();
            while len_at_call == blocks.len() {
                sleep(Duration::from_secs(1)).await;
                blocks = self.blocks();
            }
        }
        blocks
    }

    pub async fn block_sizes(&self) -> Vec<u32> {
        self.blocks_until_empty()
            .await
            .into_iter()
            .map(|block| {
                let limit =
                    block.find(" transactions").expect("Failed to find transactions in block");
                let number = block[..limit].split(' ').last().unwrap();
                number.parse::<u32>().expect("Failed to parse number of transactions")
            })
            .collect()
    }

    pub async fn block_times(&self) -> Vec<i64> {
        let mut v = self
            .blocks_until_empty()
            .await
            .into_iter()
            .map(|block| {
                let time = block.split('"').nth(3).unwrap();
                let time: DateTime<Utc> = time.parse().expect("Failed to parse time");
                time
            })
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| (w[1] - w[0]).num_milliseconds())
            .collect::<Vec<_>>();

        // First block has no previous one, so always has a time of 0
        v.insert(0, 0);
        v
    }

    pub async fn steps(&self) -> Vec<u64> {
        let matching = "Transaction resource usage: Steps: ";
        BufReader::new(File::open(&self.log_filename).unwrap())
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                if let Some(start) = line.find(matching) {
                    let end = line.find(" | ");
                    let steps = line[start + matching.len()..end.unwrap()].to_string();

                    Some(steps.parse::<u64>().unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}
