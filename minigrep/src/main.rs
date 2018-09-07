extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(conf) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
