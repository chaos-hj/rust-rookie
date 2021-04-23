#[allow(dead_code)]
fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &x;
}

#[allow(dead_code)]
pub fn it_works() {
    function::it_works();
    bound::it_works();
    coercion::it_works();
    static_lifetime::it_works();
    elision::it_works();
}
pub mod function {
    fn print<'a>(x: &'a i32) {
        println!("`print`: x is {}", x);
    }

    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    fn use_x(x: i32) {
        let _y = x + 1;
    }

    fn use_obj(x: String) {
        let _y = x;
    }

    fn borrow_to<'a>(_x: &'a String) -> &'a str {
        "jei"
    }

    #[allow(dead_code)]
    pub fn it_works() {
        let mut a = 12;
        // borrow
        print(&a);
        // mut borrow
        add_one(&mut a);
        // borrow
        print(&a);
        // move
        println!("{}", a);

        // a,b init
        let (a, b) = (10, 11);
        // a,b alive
        print_multi(&a, &b);
        // b move
        println!("{}", b);
        {
            // a copy
            println!("{}", *&a);
        }
        use_x(a);
        // a alive
        let z = pass_x(&a, &b);
        println!("{}", z);

        // s/o has the same lifetime
        let s = "aaaa".to_string();
        let _o = borrow_to(&s);
        // s dead
        use_obj(s);
        // use_obj(_o.to_string());
    }
}

pub mod bound {

    /// `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期`'a`。
    /// `T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。
    /// 另外`Ref` 的生命周期也不能超出 `'a`。
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    pub fn it_works() {
        let s: String = String::from("jock");
        let ref_to_s = Ref(&s);

        println!("s moved {}", &s);

        println!("ref_to_s {:?}", &ref_to_s);

        let x = 7;
        let ref_x = Ref(&x);
        print(x);
        print_ref(&ref_x);
    }

    fn print<T>(t: T)
    where
        T: std::fmt::Debug,
    {
        println!("print: t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T)
    where
        T: std::fmt::Debug + 'a,
    {
        println!("print_ref: t is {:?}", t);
    }
}

pub mod coercion {
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    pub fn it_works() {
        // long lifetime
        let first = 2;
        {
            // short lifetime
            let second = 3;
            println!("The product is {}", multiply(&first, &second));
            println!("{} is the first", choose_first(&first, &second))
        }
    }
}

pub mod static_lifetime {

    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    pub fn it_works() {
        {
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);
        }

        {
            let lifetime_num = 9;
            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static : {}", coerced_static);
        }
        println!("NUM: {} stays accessible!", NUM);
    }
}

pub mod elision {
    fn elided_input(x: &i32) {
        println!("elided_input : {}", x);
    }
    fn annotated_input<'a>(x: &'a i32) {
        println!("annotated_input : {}", x);
    }
    fn elided_pass(x: &i32) -> &i32 {
        x
    }
    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    pub fn it_works() {
        let x = 3;
        elided_input(&x);
        annotated_input(&x);

        println!("elided_pass : {}", elided_pass(&x));
        println!("annotated_input ; {}", annotated_pass(&x));
    }
}
