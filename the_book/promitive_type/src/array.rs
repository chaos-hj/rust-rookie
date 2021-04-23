pub fn array() {
    let mut array = [0; 3];

    println!("array is {:?}", array);

    array[0] = 1;
    println!("array is {:?}", array);

    // an array itself is not iterable
    // for x in array {}

    // use the array reference's IntoIterator implementation
    for _x in &array {}
    // coerce the array to a slice by calling a slice method
    for _x in array.iter() {}

    // user must implement Copy
    let _obj_array = [User {}; 3];

    let [hello, world] = ["hello", "world"];
    println!("{}, {}", hello, world);

    let array = ["hello", "cargo"];
    let [hello, cargo] = array;
    println!("{}, {}", hello, cargo);
    // array = ["No", "Changed"];
    // println!("{}, {}", hello, cargo);
    let str = "chaos".to_string();
    let x = std::array::from_ref(&str);
    println!("{:?}", x);
    let mut str = "chaos".to_string();
    let y = std::array::from_mut(&mut str);
    println!("{:?}", y);

    let x = [1, 2, 3];
    println!("x: {:?} ", x);
    let y = x.map(|v| v * v);
    println!("y: {:?} ", y);
}

struct User {}
use std::marker::Copy;
impl Copy for User {
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {}
    }
}
