use std::ops::Add;

#[allow(dead_code)]
pub fn it_works() {
    add::it_works();
    drop::it_works();
}

mod add {

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    impl super::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, _rhs: Bar) -> Self::Output {
            FooBar
        }
    }

    pub fn it_works() {
        let foo = Foo;
        let bar = Bar;
        let foobar = foo + bar;

        println!("{:?}", foobar);
    }
}

mod drop {
    pub fn it_works() {
        let _a = Droppable { name: "a" };
        {
            println!("In Block");
            let _b = Droppable { name: "b" };
            println!("out Block");
        }

        println!("from Block into fn-main");

        println!("manual drop _a");
        drop(_a);

        println!("end of fn-main, will exit");
    }

    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }
}

mod iterators {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }
}
