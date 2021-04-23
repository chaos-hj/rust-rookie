use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let _re = &args[1];
    let file = &args[2];
    let _content = fs::read_to_string(file).expect("failed");

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        eprintln!("problem parsing arguments : {}", err);
        process::exit(-1);
    });
    let _content = fs::read_to_string(config.file).expect("failed");
}

pub struct Config {
    re: String,
    file: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let case_sensitive = env::var("Case_insensitive").is_err();
        Ok(Config {
            re: args[1].clone(),
            file: args[2].clone(),
            case_sensitive: case_sensitive,
        })
    }

    pub fn build(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get afilename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            re: query,
            file: filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn find<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

use std::error::Error;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    for line in search(&config.re, &contents) {
        println!("{}", line);
    }
    if config.case_sensitive {
        search(&config.re, &contents);
    } else {
        search_case_insensitive(&config.re, &contents);
    }
    Ok(())
}

#[cfg(test)]
mod test;
