use clap::Parser;
use crossterm::{cursor, style::Print, ExecutableCommand, Result};
use std::collections::VecDeque;
use std::io;
use std::io::stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

/// This is a simple text scope which reads stdin and expect two input values per line
/// The first input value is epoch
/// The second is the reading to be displayed on the scope
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of columns to use
    #[clap(short, long, value_parser, default_value_t = 80)]
    columns: u16,

    /// The maximum value expected in the data column
    #[clap(short, long, value_parser, default_value_t = 256.0)]
    max_vals: f64,

    /// Number of rows to use
    #[clap(short, long, value_parser, default_value_t = 25)]
    rows: u16,

    /// Time step per column
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    step: f64,
}

fn now() -> f64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("now is not a good time");
    since_the_epoch.as_millis() as f64 / 1000.0
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 1..=args.rows {
        stdout().execute(Print("\n"))?;
    }

    stdout().execute(cursor::SavePosition)?;

    let stdin_channel = spawn_stdin_channel();
    let mut deque = VecDeque::new();
    loop {
        match stdin_channel.try_recv() {
            Ok(line) => {
                let input_values: Vec<_> = line.trim().split_whitespace().collect();
                let epoch: f64 = input_values[0].parse().unwrap();
                let value: f64 = input_values[1].parse().unwrap();
                deque.push_front((epoch, value));
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                break;
            }
        }

        let base_epoch = now() - args.step * args.columns as f64;
        loop {
            match deque.pop_back() {
                Some((epoch, value)) => {
                    if epoch > base_epoch {
                        deque.push_front((epoch, value));
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let mut display = vec![vec!['#'; args.columns as usize]; args.rows as usize];

        for elem in &deque {
            let (epoch, value) = elem;
            let seconds_since_base = epoch - base_epoch;
            let column = (seconds_since_base / args.step).clamp(0.0, args.columns as f64 - 1.0);

            let value = value.clamp(0.0, args.max_vals);
            let row = (value / args.max_vals * args.rows as f64) as usize;

            display[row][column as usize] = 'O';
        }

        stdout()
            .execute(cursor::RestorePosition)?
            .execute(cursor::MoveUp(args.rows))?;

        for line in display {
            let s: String = line.into_iter().collect();
            stdout().execute(Print(s))?;
            stdout().execute(Print("\n"))?;
        }
    }
    Ok(())
}
