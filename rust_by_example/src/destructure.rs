#[allow(dead_code)]
pub fn it_works() {
    println!("============mod destructure===========");

    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let not_a_ref = 3;
    let ref is_a_ref = 3;
    println!("not_a_ref: {}", not_a_ref);
    println!("is_a_ref: {}", is_a_ref);

    match not_a_ref {
        ref val => println!("Got a reference to a value: {:?}", val),
    }

    let num = 9;
    match num {
        0 | 10 => println!("num is 0 or 10, {}", num),
        n @ 1..=9 => println!("num in [1, 9], {}", n),
        _ => println!("num > 10, {}", num),
    }

    let a = Foo::Bar;
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    'outer: loop {
        'inner: loop {
            println!("exit innner loop");
            break 'inner;
        }
        println!("exit outer loop");
        break 'outer;
    }
}


enum Foo {
    Bar,
    _Baz,
    _Qux(u32),
}
