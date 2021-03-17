use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_nuber = 7;

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculatiuon(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculatiuon(intensity)
        );
        println!(
            "Next, do {} situps.",
            simulated_expensive_calculatiuon(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_calculatiuon(intensity)
            );
        }
    }
}

fn generate_workout1(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculatiuon(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Rememeber to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Rememeber to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn _add_one_v1(x: u32) -> u32 {
    x + 1
}

fn _test_1() {
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let _add_one_v3 = |x| { x + 1 };
    // let _add_one_v4 = |x| x + 1;
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout3(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_value() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn move_closure() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

}
