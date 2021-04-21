use std::error::Error;
use std::fmt::{self, Display, Formatter};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for EmptyVec {
    fn description(&self) -> &str {
        "invaild first item to double"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

fn double_first(vec: &Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
}

fn print(vec: &Vec<&str>, result: Result<i32>) {
    match result {
        Ok(n) => println!("The first of {:?} doubled is {}", vec, n),
        Err(e) => println!("Error: {}", e),
    }
}

fn _do_double_first(vec: Vec<&str>) -> Result<i32> {
    // chain
    let first = vec.first().ok_or(EmptyVec)?.parse::<i32>()?;
    Ok(2 * first)
}

pub fn it_works() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(&numbers, double_first(&numbers));
    print(&empty, double_first(&empty));
    print(&strings, double_first(&strings));
}
