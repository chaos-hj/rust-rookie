use crate::learn_box::List::{Cons, Nil};
use crate::learn_box::*;
use crate::learn_deref::*;
use crate::learn_drop::*;
use crate::learn_rc::test_rc;
use crate::learn_refcell::rc_refcell;
fn main() {
    test_box();

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    println!("{:?}", list);

    it_work();
    do_something();
    deref_coercions();
    learn_drop::do_drop();

    let c = CustomSmartPointer { data: "make nosie".to_string() };
    println!("pointer created.");
    drop(c);
    println!("pointer dropped before the end of main");
    test_rc();
    rc_refcell();
}

mod learn_box;
mod learn_deref;
mod learn_drop;
mod learn_rc;
mod learn_refcell;