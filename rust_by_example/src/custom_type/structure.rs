pub fn it_works() {
    let name = "peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let _nil = Nil;
    let p1 = Point{x: 1.0, y: 10.0};
    let p2 = Point{x: 2.0, ..p1};
    println!("p1:{:?}, p2:{:?}", p1, p2);
    let Point{x, y} = p2;
    let p3 = Point {x, y};
    let pair = Pair(1.0, 23.0);
    let Pair(x, y) = pair;
    let p4 = Point {x, y};
    let rectangle = Rectangle {p1: p3, p2: p4};
    println!("{:?}", rectangle);
}

// C struct
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// unit struct
struct Nil;

// tuple struct
struct Pair(f32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
