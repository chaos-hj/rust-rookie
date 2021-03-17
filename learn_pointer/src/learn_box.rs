pub fn test_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}