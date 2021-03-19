use super::learn_box::List;

fn test() {
    let a = List::Cons(5, Box::new(List::Nil));
    let b = List::Cons(4, Box::new(a));
    // let c = Cons(3, Box::new(a));
}

enum Set {
    Constructor(i32, Rc<Set>),
    Nil,
}

use std::rc::Rc;
use Set::*;
pub fn test_rc() {
    let a = Rc::new(Constructor(5, Rc::new(Set::Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Constructor(4, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Constructor(5, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
