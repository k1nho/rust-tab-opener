use rust_tab_opener::{run, Mode};
use std::env;

fn main() {
    let args = env::args();
    run(Mode::Exec);
}
