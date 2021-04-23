#[allow(dead_code)]
pub fn it_works() {
    //uncopyable
    let a = "hello";
    let mut b = "kelli".to_owned();

    let diary = || {
        println!("I said {}", a);
        b.push_str("!!!");
        println!("{}", b);
        std::mem::drop(b);
    };

    apply(diary);

    let double = |p| 2 * p;
    println!("3 * 2 = {}", apply_ref(double));
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_ref<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}