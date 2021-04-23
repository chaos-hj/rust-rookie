use std::thread;
use std::time;
pub fn thread_spawn() {
    thread::spawn(|| {
        for i in 1..7 {
            println!(
                "At {:?}, hi number {} from the spawned thread!",
                time::SystemTime::now(),
                i
            );
            thread::sleep_ms(1);
        }
    });
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!(
                "At {:?}, hi number {} from the handle thread!",
                time::SystemTime::now(),
                i
            );
            thread::sleep_ms(1);
        }
    });

    for i in 1..3 {
        println!(
            "At {:?}, hi number {} from thr mian thread!",
            time::SystemTime::now(),
            i
        );
        thread::sleep_ms(1);
    }

    handle.join().unwrap();
}

pub fn move_closures() {
    let v = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
