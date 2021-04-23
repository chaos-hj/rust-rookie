#[allow(dead_code)]
pub fn it_works() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}


fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a {}", text)
}