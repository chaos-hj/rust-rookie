pub mod array;
pub mod literal;
pub mod tuple;

#[allow(dead_code)]
pub fn it_works() {
    println!("=======primitive mod============");
    literal::it_works();
    tuple::it_works();
    array::it_works();
}
