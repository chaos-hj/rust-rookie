#[allow(dead_code)]
pub fn it_works() {
    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("{}", ref_c1);
    println!("{}", ref_c2);

    let point = Point {
        x: 0,
        y: 0,
        c: Color,
    };

    let ref ref_c1 = point;
    let ref_c2 = &point;
    println!("{:?}", ref_c1);
    
    println!("point.c = {:?}", ref_c1.c);
    println!("{:?}", *ref_c1);
    println!("point.c = {:?}", (*ref_c1).c);
    println!("{:?}", ref_c2);
    println!("{:?}", *ref_c2);

    let cube = Point {
        x: 1,
        y: 1,
        c: Color,
    };
    let ref ref_c1 = cube;
    println!("{:?}", ref_c1);
    println!("cube.c = {:?}", ref_c1.c);
    println!("*cube.c = {:?}", (*ref_c1).c);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    c: Color,
}

#[derive(Debug)]
struct Color;

#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: &'static Color,
}
