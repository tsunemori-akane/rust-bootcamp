use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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

    #[serde(rename = "final_rankings__points")]
    rankings_points: u32,
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version)]
pub struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "convert csv to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')] // "output.json".into()
    pub delimiter: char,

    #[arg(long, default_value_t = true)] // "output.json".into()
    pub header: bool,
}
// cargo run -- csv -i test.csv
// cargo run csv -i assets/final_rankings_2021.csv
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize::<Record>() {
                let record = result?;
                // println!("{:?}", record);
                ret.push(record);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(&opts.output, json)?;
        }
    }
    Ok(())
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
