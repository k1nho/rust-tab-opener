use serde::{Deserialize, Serialize};
use std::fs;

pub enum Mode {
    Exec,
    Write,
}

enum Count {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Debug)]
struct Series {
    name: String,
    ep: usize,
    limit: usize,
}

impl Series {
    fn print(&self) {
        println!(
            "the serie is {}, has {} episodes , with limit {}",
            self.name, self.ep, self.limit
        );
    }
}

pub fn run(mode: Mode) {
    //get Series
    let file_contents = fs::read_to_string("./series.json").unwrap();
    let mut series: Vec<Series> = serde_json::from_str(&file_contents).unwrap();

    update_count(series, Count::Up);
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
        }
        Count::Down => {
            updated_series = series
                .into_iter()
                .map(|mut s| {
                    s.ep -= 1;
                    s
                })
                .collect();
        }
    }

    write_to_json(updated_series);
}

fn write_to_json(series: Vec<Series>) {}
