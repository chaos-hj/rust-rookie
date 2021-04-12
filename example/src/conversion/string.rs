pub fn it_works() {
   let pair = Pair(1, 2);
   println!("{}", pair.to_string());

   let s = "[3,4]";
   let pair = Pair::from_str(&s);
   println!("{:?}", pair);
}
use std::string::{ToString};
use std::str::FromStr;

#[derive(Debug)]
struct Pair(i32, i32);

impl ToString for Pair {
    fn to_string(&self) -> String {
        format!("[{}, {}]", self.0, self.1)
    }
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.replace("[", "").replace("]", "");
        let vec: Vec<&str> = x.split(",").collect();
        if vec.len() == 2 {
            Ok(Pair((*vec[0]).parse::<i32>().unwrap(), (*vec[1]).parse::<i32>().unwrap()))
        } else {
            Err(())
        }
    }
}

