#![allow(dead_code)]
use WebEvent::*;
pub fn it_works() {
    let pressed = KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("zero is {}", Number::Zero as i32);
    println!("Red is {:06x}", Color::Red as i32);

    let mut list = List::new();
    list = list.prepend(32);
    list = list.prepend(64);
    println!("len: {}, [{}]", list.len(), list.stringfy());
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

// implicit
enum Number {
    Zero,
    One,
}

// explicit
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

use List::*;

impl List {
    fn new() -> Self {
        Nil
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref next) => 1 + next.len(),
            Nil => 0,
        }
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn stringfy(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringfy())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}
