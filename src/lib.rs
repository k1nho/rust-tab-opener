use serde::{Deserialize, Serialize};
use std::fs;
use std::process;
use std::thread;
use std::time;

#[derive(Debug)]
pub enum Mode {
    Exec,
    Write,
}

#[derive(Debug)]
pub enum Count {
    Up,
    Down,
    Neutral,
}

#[derive(Serialize, Deserialize, Debug)]
struct Series {
    name: String,
    ep: usize,
    limit: usize,
}

#[derive(Debug)]
pub struct Config {
    mode: Mode,
    count: Count,
}

impl Series {
    fn print(&self) {
        println!(
            "{} on episode {}, expected: {}\n",
            self.name, self.ep, self.limit
        );
    }
}

pub fn config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let mode = match args.next() {
        Some(arg) if &arg == "x" => Mode::Exec,
        Some(arg) if &arg == "w" => Mode::Write,
        _ => Mode::Exec,
    };

    let count = match args.next() {
        Some(count) if &count == "u" => Count::Up,
        Some(count) if &count == "d" => Count::Down,
        _ => Count::Neutral,
    };

    Ok(Config { mode, count })
}

pub fn run(config: Config) {
    //get Series
    let file_contents = fs::read_to_string("./series.json").unwrap();
    let series: Vec<Series> = serde_json::from_str(&file_contents).unwrap();

    match config.mode {
        Mode::Exec => {
            open_tabs(series);
        }
        Mode::Write => match config.count {
            Count::Up => {
                update_count(series, Count::Up);
            }
            Count::Down => {
                update_count(series, Count::Down);
            }
            Count::Neutral => {
                println!("*************************************\n");
                for s in series {
                    s.print();
                }
                println!("*************************************");
            }
        },
    }
}

fn update_count(series: Vec<Series>, count: Count) {
    let updated_series: Vec<Series>;
    match count {
        Count::Up => {
            updated_series = series
                .into_iter()
                .map(|mut s| {
                    s.ep += 1;
                    s
                })
                .filter(|s| s.limit != s.ep)
                .collect();
            write_to_json(updated_series);
        }
        Count::Down => {
            updated_series = series
                .into_iter()
                .map(|mut s| {
                    s.ep -= 1;
                    s
                })
                .collect();
            write_to_json(updated_series);
        }
        Count::Neutral => (),
    }
}

fn write_to_json(series: Vec<Series>) {
    // needs to update series.json to be the new updated series
    let json_series = serde_json::to_string_pretty(&series).unwrap();
    fs::write("series.json", json_series);
}

fn open_tabs(series: Vec<Series>) {
    // spawn concurrent child processes for each serie
    for serie in series {
        thread::spawn(move || {
            serie.print();
            // mac command
            process::Command::new("open")
                .args(["-a", "Google Chrome", "http://google.com"])
                .spawn()
                .expect("command failed to start");
        });
    }

    // sleep for now (substitute for actual join)
    thread::sleep(time::Duration::from_secs(5));
}
