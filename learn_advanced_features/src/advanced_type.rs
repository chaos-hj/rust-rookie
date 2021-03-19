
fn thunk_type() {
    type Thunk = Box<dyn Fn() + Send + 'static>;
}


pub fn it_works() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
