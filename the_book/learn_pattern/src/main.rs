fn main() {
    if_let();
    while_let();
    muti_pattern();
    range_pattern();
    destruct();
}

fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("age > 30");
        } else {
            println!("age <= 30");
        }
    } else {
        println!("none pattern match, use default");
    }
}

fn while_let() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn muti_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other number"),
    } 
}

fn range_pattern() {
    let x = 5;
    match x {
        1..=5 => println!("nubmer between 1 and 5"),
        _ => println!("other number"),
    }
}

#[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

fn destruct() {
    let white = Color{red: 255, green: 255, blue: 255};
    let Color{red, green, blue: _} = white;

    let yellow = Color{red, green, blue: 0};

    let tuple = (1, 2, 3, 4);
    let (.., last) = tuple;
    let (_, second, ..) = tuple;

    println!("yellow rgb is {:?}", yellow);

    println!("tuple is {:?}, the second element is {}, the last element is {}", tuple, second, last);
}
