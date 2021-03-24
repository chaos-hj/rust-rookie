// proc-macro (function-like)
// use hello_macro::my_proc_macro;

// pub trait HelloMacro {
//     fn hello_macro();
// }
// use hello_macro::HelloMacro;

use hello_macro::Show;
// ...
#[derive(Show)]
struct MySelf {
    name: String,
    age: u8,
}
// ...

pub fn it_works() {
    let me = MySelf{name: "Jamie", age: 255};
    println!("{}", me); // MySelf (Jamie, 255)
    // my_proc_macro!(hello)!;
}