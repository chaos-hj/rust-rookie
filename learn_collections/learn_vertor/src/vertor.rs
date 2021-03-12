pub fn build() {
    let mut v: Vec<i32> = Vec::new();
    {
        let mut v = vec![1, 2, 3];
        {
            for i in v {
                println!("{}", i);
            }
        }
        // v.push(4); // borrow of moved value
        // println!("{}", v.len());
    }
    v.push(5);
    println!("{}", v.len());

    if v.get(10).is_none() {
        println!("The 11th element is none!");
    }
    // let x = &v[10]; // panic
    v.push(6);
    let x = &v[0]; // immutable borrow
                   // v.push(7); // mutable borrow
    println!("Vertor first element is {}", x);

    println!("for i in &mut v");
    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    println!("for &i in &v ");
    for &i in &v {
        println!("{}", i);
    }

    println!("for i in &v ");
    for i in &v {
        println!("{}", i);
    }

    println!("for i in v ");
    for i in v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.8),
        SpreadsheetCell::Text("chaos".to_string()),
    ];

    for j in &row {
        println!("The {:?} element is {:?}", j, &j);
    }
   
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}




