// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, args: i32) -> i32 {
    f(f(args))
}

fn fn_point_and_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers.iter()
    .map(|i| i.to_string())
    .collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers.iter()
    .map(ToString::to_string)
    .collect();
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x)
}

pub fn it_works() {
    let answer = do_twice(add_one, 5);
    println!("5 do twice add_one is {}", answer);
}