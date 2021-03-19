pub fn _borrow_rule() {
    let _x = 5;
    // let y = &mut x;
}

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messager.send("over max");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("over 90% of max");
        } 
    }
}

use std::cell::RefCell;
pub struct MockMessager {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessager {
    pub fn new() -> MockMessager {
        MockMessager { sent_messages: RefCell::new(vec![]) }
    } 
}

impl Messager for MockMessager {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

use std::rc::Rc;
#[derive(Debug)]
enum Map {
    Cons(Rc<RefCell<i32>>, Rc<Map>),
    Nil
}

use Map::{Cons, Nil};

pub fn rc_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    println!("a before is {:?}", a);
    println!("b before is {:?}", b);
    println!("c before is {:?}", c);

    *value.borrow_mut() += 10;
    println!("value +10");

    println!("a after is {:?}", a);
    println!("b after is {:?}", b);
    println!("c after is {:?}", c);
}