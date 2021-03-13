use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let re = &args[1];
    let file = &args[2];
    let content = fs::read_to_string(file).expect("failed");

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(-1);
    });
    let content = fs::read_to_string(config.file).expect("failed");
}

struct Config {
    re: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        Ok(Config {
            re: args[1].clone(),
            file: args[2].clone(),
        })
    }
}
