fn main() {
    println!("Hello, world!");
    let x = 'z';
    let y = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}{}{}", x, y, heart_eyed_cat);

    //tuple
    let tup = ("hello", heart_eyed_cat);
    let (x, _y) = tup;
    let tup_item1 = tup.1;
    println!("{} {}", x, tup_item1);

    //array

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    let a = [3; 5];
    println!("{}", a[0]);
    let a = [3, 3, 3, 3, 3];
    println!("{}", a[0]);

    iter(2, 6);

    //move
    let s1 = String::from("hello");
    let size1 = doSomething(s1);
    println!("{} length {}", s1, size1);
    //borrow
    let s2 = String::from("world");
    let size2 = doSomething2(&s2);
    println!("{} length {}", s2, size2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    println!("{} and {}", r1, r2);
    let r3 = &mut s; 
    println!("{}", r3);
}

fn iter(x: i32, y: i32) {
    println!("Run iter fn");
    for number in (x..y).rev() {
        println!("{}", number);
    }
    println!("End iter fn");
}

fn doSomething(s: String) -> usize {
    let len = s.len();
    len
}

fn doSomething2(s: &String) -> usize {
    s.len()
}

/**
 *this function's return type contains a borrowed value, but there is no value for it to be borrowed from
 *help: consider using the `'static` lifetime: `&'static
*/
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
