enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddrEnum1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;

    route(IpAddrKind::V4);

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let localhost = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    let localhost = IpAddrEnum1::V4(127, 0, 0, 1);

    let m = Message::Quit;
    m.call();
}

fn route(ip_type: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("hello");
    }
}

fn option_test() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    let z = x + y.expect("y is none");

    let mut x = Some(2);

    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


fn plus_one(x: Option<i32>, msg: &str) -> i32 {
    match x {
        None => x.expect(msg),
        Some(v) => v+1,
    }
}

fn match_under() {
    let some_u8_number = 0u8;

    match some_u8_number {
        1 => println!("one"),
        _ => (),
    }
}

fn let_match() {
    let mut some_u8_number = Some(0u8);
    match some_u8_number {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_u8_number {
        println!("Three");
    } else {
        some_u8_number = None;
    }
}
