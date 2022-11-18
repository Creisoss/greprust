extern crate greprust;

use std::process;
use std::env;
use greprust::Config;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Problem parsing arguments: {}", err).expect("Could not write to stderr");
        process::exit(1);
    });
    
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);
    
    if let Err(e) = greprust::run(config) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }
}
