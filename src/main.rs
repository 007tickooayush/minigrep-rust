use std::{env, error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];

    let config = Config::new(&args).unwrap_or_else( | err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config,&str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); // clone() is used to make a copy of the string
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}
