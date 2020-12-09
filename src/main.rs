use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = parse_name(&args).unwrap_or_else(|err| {
        println!("Problem reading name: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(name) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn parse_name(args: &[String]) -> Result<&[String], &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }

    Ok(&args[1..])
}
