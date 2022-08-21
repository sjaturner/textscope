use clap::Parser;
use crossterm::{cursor, style::Print, ExecutableCommand, Result};
use signal_hook::flag;
use std::collections::VecDeque;
use std::io;
use std::io::stdout;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match tx.send(buffer) {
            Ok(_) => { },
            Err(_) => { std::process::exit(0) },
        }
    });
    rx
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 1..=args.rows {
        stdout().execute(Print("\n"))?;
    }

    stdout().execute(cursor::SavePosition)?;

    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    let stdin_channel = spawn_stdin_channel();
    let mut deque = VecDeque::new();
    loop {
        match stdin_channel.try_recv() {
            Ok(line) => {
                let input_values: Vec<_> = line.trim().split_whitespace().collect();

                if input_values.len() != 2 {
                    break;
                }
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

        let mut display = vec![vec![' '; args.columns as usize]; args.rows as usize];

        for elem in display[0].iter_mut() {
            *elem = '-';
        }

        let last_row_index = (args.rows - 1) as usize;
        for elem in display[last_row_index].iter_mut() {
            *elem = '-';
        }

        let last_column_index = (args.columns - 1) as usize;
        for row in 0..args.rows {
            display[row as usize][last_column_index] = '|';
        }

        display[0][last_column_index] = '+';
        display[last_row_index][last_column_index] = '+';

        for elem in &deque {
            let (epoch, value) = elem;
            let seconds_since_base = epoch - base_epoch;
            let column = (seconds_since_base / args.step).clamp(0.0, args.columns as f64 - 1.0);
            let value = value.clamp(0.0, args.max_vals);
            let row = value / args.max_vals * args.rows as f64;

            display[row as usize][column as usize] = '+';
        }

        if term.load(Ordering::Relaxed) {
            break;
        }

        stdout()
            .execute(cursor::RestorePosition)?
            .execute(cursor::MoveUp(args.rows))?;

        for line in display {
            let s: String = line.into_iter().collect();
            stdout()
                .execute(Print(s))?
                .execute(cursor::MoveDown(1))?
                .execute(cursor::MoveToColumn(0))?;
        }
    }

    stdout().execute(cursor::RestorePosition)?;

    Ok(())
}
