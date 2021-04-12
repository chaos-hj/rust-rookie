#[allow(dead_code)]
pub fn it_works() {
    println!("============mod function===========");
    let haystack = vec![1, 2, 3];
    println!("{:?}", &haystack);
    let contains = move |needle| haystack.contains(needle);
    println!("contains 1 {}", contains(&1));

   input::it_works();
   output::it_works();
   hof::it_works();
}

mod input;
mod output;
// high order function
mod hof;

