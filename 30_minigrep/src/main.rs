


use minigrep::{Config,run};
use std::{env, process};
fn  main() {
    let args: Vec<String> = env::args().collect();
    /*
    [src\lib:4:5] args = [
    "target\\debug\\minigrep.exe",
    "needle",
    "sds",
        ]
     */
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e)=run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}

