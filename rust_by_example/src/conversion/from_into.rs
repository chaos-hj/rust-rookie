pub fn it_works() {
    let num = Number::from(64);
    println!("{:?}", num);
    let num: Number = 32.into();
    println!("{:?}", num);

    println!("TryFrom");
    let num = EvenNumber::try_from(8);
    println!("{:?}", num);
    let num = EvenNumber::try_from(5);
    println!("{:?}", num);
    let num: Result<EvenNumber, i32> = 128_i32.try_into();
    println!("{:?}", num);
}
use std::convert::{From, TryFrom, TryInto};
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(value)
        }
    }
}
