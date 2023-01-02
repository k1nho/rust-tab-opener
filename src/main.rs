use rust_tab_opener::{config, run};
use std::env;

fn main() {
    let args = env::args();
    let config = config(args).unwrap();
    println!("config is {:?}", config);
    // run(config)
}
