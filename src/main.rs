use rust_tab_opener::{config, run};
use std::env;
use std::time::Instant;

fn main() {
    let args = env::args();
    let config = config(args).unwrap();
    let start = Instant::now();
    run(config);
    let duration = start.elapsed();
    println!("it took {:?} time", duration);
}
