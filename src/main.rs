use std::env;
use std::process;
use std::fs;
use std::error::Error;

mod lib;

const ANIMALS_FILE: &str = "./animals.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    let (name, range) = parse_inputs(&args).unwrap_or_else(|err| {
        println!("Problem reading name: {}", err);
        process::exit(1);
    });

    let animals = load_animals().unwrap_or_else(|e| {
        println!("Error loading file: {}", e);
        process::exit(1);
    });

    let res = lib::run(&name, &animals).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });

    print_animals(res, range);
    process::exit(0);
}

fn parse_inputs(args: &[String]) -> Result<(&[String], i32), Box<dyn Error>> {
    if args.len() < 3 {
        return Err("Not enough arguments".into());
    }

    let range: i32 = args[1].parse()?;

    Ok((&args[2..], range))
}

// Load list of animals from file
fn load_animals() -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(ANIMALS_FILE)?.to_lowercase();
    Ok(result)
}


fn print_animals(set: Vec<(String, usize)>, range: i32) {
    let mut rank = 0;
    let mut count = 0;
    for animal in set {
        if animal.1 != 0 {
            
            if animal.1 != rank {
                count += 1;
                if count > range {
                    break;
                }
                println!();
                println!("Animals with rank: {}", animal.1);
                rank = animal.1;
            }

            println!("{}", animal.0);
        }
    }
}