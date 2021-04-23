type Thunk = Box<dyn Fn() + Send + 'static>;
fn thunk_type(fun: Thunk) -> Thunk {
    fun();
    Box::new(|| println!("use thunk return"))
}

fn diverging_fn() {
    let guess = String::from("11");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}

fn dyn_sized_type() {
    // let s1: str = "can't work here!";
}

fn generic<T>(_t: T){}

//Sized trait
fn equal_generic<T: Sized>(_t: T) {}

//Sized trait bound 
fn bound_generic<T: ?Sized>(_t: &T) {}

pub fn it_works() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    let fun = thunk_type(Box::new(|| println!("use thunk input")));
    fun();
}
