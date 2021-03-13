use std::fs::File;
use std::io::ErrorKind;
fn main() {

    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("failed msg:{:?}", error);
        },
    };

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed creating the file : {:?}", e),
            },
            other_error => panic!("other reason: {:?}", error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed creating the file : {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();
    let v = vec![1, 2, 3];
    v[99];
}

use std::io::Error;
use std::io::Read;

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;
fn read_username_from_file3() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}


fn read_file() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");
    Ok(())
}

