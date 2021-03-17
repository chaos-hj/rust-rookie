use super::*;

fn test() {
    let a = Cons(5, Box::new(List::Nil));
    let b = Cons(4, Box::new(a));
    // let c = Cons(3, Box::new(a));
}

enum Set {
    Constructor(i32, Rc<Set>),
    Nil,
}

use std::rc::Rc;
use Set::*;
fn test_rc() {
    let a = Rc::new(Constructor(5, Rc::new(Set::Nil)));
    let b = Constructor(4, Rc::clone(&a));
}