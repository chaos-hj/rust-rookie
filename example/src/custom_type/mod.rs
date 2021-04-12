pub mod enumeration;
pub mod structure;
pub mod constant;

#[allow(dead_code)]
pub fn it_works() {
    println!("===============mod custom_type=============");
    structure::it_works();
    enumeration::it_works();
    constant::it_works();
}