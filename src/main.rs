use clap::Parser;
use crossterm::{cursor, style::Print, ExecutableCommand, Result};
use std::io::stdout;
use std::time::{SystemTime, UNIX_EPOCH};

/// This is a simple text scope which reads stdin and expect two input values per line
/// The first input value is epoch
/// The second is the reading to be displayed on the scope
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of columns to use
    #[clap(short, long, value_parser, default_value_t = 80)]
    columns: u8,

    /// The maximum value expected in the data column
    #[clap(short, long, value_parser, default_value_t = 256.0)]
    max_vals: f32,

    /// Number of rows to use
    #[clap(short, long, value_parser, default_value_t = 25)]
    rows: u8,

    /// Time step per column
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    step: f32,
}

fn now() -> f64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("now is not a good time");
    since_the_epoch.as_millis() as f64 / 1000.0
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 1..=args.rows {
        stdout().execute(Print("\n"))?;
    }
    stdout()
        .execute(cursor::SavePosition)?
        .execute(cursor::MoveUp(3))?
        .execute(Print("Hello"))?
        .execute(cursor::RestorePosition)?;

    for line in std::io::stdin().lines() {
        let input = line.unwrap();
        let input_values: Vec<_> = input.split(' ').collect();
        let epoch: f64 = input_values[0].parse().unwrap();
        let value: f64 = input_values[1].parse().unwrap();

        println!("{}: {} {} {}", now(), epoch, value, input_values[0]);
    }
    Ok(())
}
