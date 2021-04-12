mod cast;
mod inference;
mod alias;

#[allow(dead_code)]
pub fn it_works() {
    println!("===============mod types===============");
    cast::it_works();
    inference::it_works();
    alias::it_works();
}