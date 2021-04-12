#[allow(dead_code)]
pub fn it_works() {
    println!("============mod expression===========");
   
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };

    #[allow(unused_must_use)]
    let z = {
        2 * x;
    };

    println!("x is {}", x);
    println!("y = x + x^2 + x^3 is {}", y);
    println!("z is {:?}", z);
}