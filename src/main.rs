use std::env;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let config = Config::build(&args).expect("parse args failed");

    run(config);
}
