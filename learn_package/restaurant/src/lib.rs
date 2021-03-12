
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
        fn seat_at_table() {
            println!("seat at table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take order");
        }
        fn server_order() {
            println!("server order");
        }
        fn take_payment() {
            println!("take payment");
        }
    }
}


pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path 

    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_summer() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    let order = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting;

pub fn eat_breakfast() {
    hosting::add_to_waitlist();
}

mod meal;
use crate::meal::{lunch, choose_meal};
fn main() {
   lunch::eat_lunch();
   choose_meal();
}