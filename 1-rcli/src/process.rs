use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
struct Record {
    #[serde(rename(
        serialize = "final_rankings",
        deserialize = "final_rankings__standing_player"
    ))]
    final_rankings: String,

    nationality: String,

    #[serde(rename = "standing_player2")]
    player_name: String,

    #[serde(rename(serialize = "age", deserialize = "final_rankings__age"))]
    age: u32,

    #[serde(rename(serialize = "ranking_points", deserialize = "final_rankings__points"))]
    rankings_points: u32,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize::<Record>() {
        let record = result.unwrap();
        // println!("{:?}", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
