fn main() {

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(5);

    let s = gives_ownership();

    let s1 = takes_and_gives_back(s);

    let (s2, len) = calculate_length(s1);

    let len = calculate_length1(&s2);

    let mut s3 = String::from("Hello");
    change(&mut s3);

    {
        let r1 = &mut s3;
    }
    let r2 = &mut s3;
    println!("{}", r2);
}

fn takes_ownership(some_string: String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("get String {}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length1(s: &String) -> usize {
    s.len()
}

fn change(a_string: &mut String) {
    a_string.push_str(" world");
}