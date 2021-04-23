use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

type AliasedResult<T> = Result<T, ParseIntError>;
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn multiply_str(s1: &str, s2: &str) -> AliasedResult<i32> {
    let n1 = s1.parse::<i32>()?;
    let n2 = s2.parse::<i32>()?;
    // in the 2018 edition deprecated
    // let n2 = try!(s2.parse::<i32>());
    Ok(n1 * n2)
}

type MyType = Result<Option<i32>, ParseIntError>;

fn double_first(vec: &Vec<&str>) -> MyType {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

#[allow(dead_code)]
pub fn it_works() {
    let twenty = multiply("10", "2");
    print(twenty);
    let twenty = multiply_str("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
    let tt = multiply_str("t", "2");
    print(tt);

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!(
        "The first of {:?} doubled is {:?}",
        &numbers,
        double_first(&numbers)
    );
    println!(
        "The first of {:?} doubled is {:?}",
        &empty,
        double_first(&empty)
    );
    println!(
        "The first of {:?} doubled is {:?}",
        &strings,
        double_first(&strings)
    );

    box_err::it_works();
    iter_err::it_works();
}

mod box_err;
mod iter_err;
