use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};
// cargo run -- csv -i test.csv
// cargo run csv -i assets/final_rankings_2021.csv
// cargo run -- csv -i assets/final_rankings_2021.csv --format yaml
// cargo run genpass -l 32
// cargo run -- base64 encode --format urlsafe -i Cargo.toml
// cargo run -- base64 decode
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
