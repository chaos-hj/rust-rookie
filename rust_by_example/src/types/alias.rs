pub fn it_works() {
    let x: Inch = 300;
    let y: u64_t = 100;
    println!("{} + {} = {}", x, y, x + y);
}

type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;
