#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    use super::Rectangle;
    #[test]
    fn larger_can_hold_smaller() {
        let large = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 5,
        };
        assert!(large.can_hold(&smaller));
    }

    use super::*;
    #[test]
    fn it_adds_one() {
        assert_eq!(add_one(1), 2);
    }
    #[test]
    fn it_adds_two() {
        assert_ne!(add_one(add_one(1)), 2);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_ruslut() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("2+2!=4".to_string())
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_one(a: i32) -> i32 {
    a + 1
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}