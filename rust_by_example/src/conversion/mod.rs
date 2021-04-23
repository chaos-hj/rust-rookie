pub mod from_into;
mod string;

#[allow(dead_code)]
pub fn it_work() {
    println!("=======conversion mod================");
    from_into::it_works();
    string::it_works();
}
