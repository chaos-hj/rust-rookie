#[allow(dead_code)]
pub fn it_works() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is {}", i, data_segment);
        children.push(std::thread::spawn(move || -> u32 {
            data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum()
        }))
    }

    let mut intermediate_sums = vec![];
    for child in children {
        intermediate_sums.push(child.join().unwrap());
    }

    println!("sum is {}", intermediate_sums.iter().sum::<u32>());

    mpsc::it_works();
}

mod mpsc {
    use std::sync;
    use std::sync::mpsc::{Receiver, Sender};
    pub fn it_works() {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = sync::mpsc::channel();

        for id in 0..=3 {
            let thread_tx = tx.clone();
            std::thread::spawn(move || {
                thread_tx.send(id).unwrap();
                println!("thread {} finished", id);
            });
            // thread.join().unwrap();
        }
        drop(tx);
        for recv in rx {
            println!("recive {}", recv);
        }
    }
}
