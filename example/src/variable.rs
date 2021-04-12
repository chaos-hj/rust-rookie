#[allow(dead_code)]
pub fn it_works() {
    println!("============mod Variable===========");
    let a;
    {
        let x = 2;
        a = x * x;
        println!("a is {}", a)
    }
}