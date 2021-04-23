#[allow(dead_code)]
pub fn it_works() {
    // implicit
    let a = generic(SGen(100i8));
    println!("Debug:{:?}", a);
    // explicit
    let b = generic::<char>(SGen('c'));
    println!("Debug:{:?}, To_String:{}", &b, &b.to_string());
}

#[derive(Debug)]
struct SGen<T>(T);
fn generic<T>(t: SGen<T>) -> SGen<T> {
    t
}

impl SGen<char> {
    fn to_string(&self) -> String {
        (self.0).to_string()
    }
}


mod assoc_item;
mod phanton_type;