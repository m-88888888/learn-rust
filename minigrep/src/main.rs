extern crate minigrep;

use std::env;
use std::process; // I/Oに関するトレイトを使うためのインポート

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(r) = minigrep::run(config) {
        println!("Application error: {}", r);

        process::exit(1);
    }
}
