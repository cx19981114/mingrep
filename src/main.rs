use std::env;
use std::process;

use mingrep::Config;

fn main() {
    // let args = env::args();
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parseing arguments:{}",err);
        process::exit(1);
    });
    if let Err(e) = mingrep::run(config){
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
}
