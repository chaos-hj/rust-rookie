#[allow(unused)]
pub fn it_works() {
    sort_numberic();
    sort_struct();
}

fn sort_numberic() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    println!("{:?}", vec);

    let mut vec = vec![1.1, 1.6, 5.5, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", vec);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8,
}


fn sort_struct() {
    let mut people = vec![
        Person {name: "Zoe".to_string(), age: 25},
        Person {name: "Zoe".to_string(), age: 20},
        Person {name: "Jacky".to_string(), age: 20},
        Person {name: "Al".to_string(), age: 66},
        Person {name: "John".to_string(), age: 7},
    ];
    people.sort();

    println!("{:?}", people);
}
