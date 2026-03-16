use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;
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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result.unwrap();
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let contect = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, contect)?;
    Ok(())
}
