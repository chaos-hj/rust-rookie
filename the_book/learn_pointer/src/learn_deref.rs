pub fn it_work() {
    let x = 5;
    let y = &x;
    let y = Box::new(x);

    println!("x is {}", x);
    println!("y = &x, *y is {}", *y);
    println!("y = Box::new(x), *y is {}", *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn do_something() {
    let x = 5;
    let y = MyBox::new(x);
    println!("y is {}", *y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn deref_coercions() {
    let r = MyBox::new(String::from("Rust"));
    let j = "java";
    let g = String::from("golang");
    let p = Box::new("python");
    hello(&r);
    hello(j);
    hello(&g);
    hello(&p);
}