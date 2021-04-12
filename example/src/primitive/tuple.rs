pub fn it_works() {
    let pair = (32, true);
    println!("{:?} reversed is {:?}", pair, reverse(pair));

    let m = Martix(1.0, 2.0, 3.0, 4.0);
    println!("Debug: {:?}", m);
    println!("Display:\n{}", m);
    println!("Display Transpose:\n{}", m.transpose());
}

fn reverse(pair: (i8, bool)) -> (bool, i8) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Martix(f32, f32, f32, f32);

use std::fmt::{Display, Formatter, Result};

impl Display for Martix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (a, b, c, d) = (&self.0, &self.1, &self.2, &self.3);
        let t1 = (a, b);
        let t2 = (c, d);
        write!(f, "{:?}\n{:?}", t1, t2)
    }
}

impl Martix {
    fn transpose(&self) -> Self {
        Martix(self.0, self.2, self.1, self.3)
    }
}
