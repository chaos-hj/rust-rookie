macro_rules! say_hello {
    () => {
        println!("Say Hello By Macro!");
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! calculate {
    (eval $e:expr) => {
        let val:usize = $e;
        println!("{} = {}", stringify!($e), val);
    };

    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate!{ eval $e }
        calculate!{ $(eval $es),+ }
    }};
}

#[allow(dead_code)]
pub fn it_works() {
    say_hello!();
    foo();
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(1i32+ 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}", stringify!($func_name))
        }
    };
}

create_function!(foo);
