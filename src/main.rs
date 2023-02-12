use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", _config.query);
    println!("In file {}", _config.file_path);

    if let Err(e) = minigrep::run(_config) {
        // pass
    }

}