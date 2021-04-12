pub mod debug;
pub mod display;
pub mod fmt;

#[allow(dead_code)]
pub fn it_work() {
    println!("=======print mod================");
    debug::it_works();
    display::it_works();
    fmt::it_works();
}
