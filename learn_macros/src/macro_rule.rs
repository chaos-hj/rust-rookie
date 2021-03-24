#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is my macro!");
    };
}

#[macro_export]
macro_rules! my_macro_with_args {
    ($($x:expr),*) => {
        let mut i = 1;
        $(
            println!("The {} arg is {}", i, $x);
            i += 1;
        )*
    };
}

pub fn it_works() {
    my_macro!();
    my_macro_with_args![10, 20];
    my_macro_with_args!(10, 20);
}
