use minigrep::{Config, parse_config};

use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
