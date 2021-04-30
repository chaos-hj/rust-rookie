fn main() {
    crossbeam::it_works();
}

mod crossbeam {
    #![allow(unused)]
    use crossbeam_channel::{bounded, unbounded};
    use rand::prelude::*;
    use std::thread;
    use std::time::{Duration, Instant};

    pub fn it_works() {
        // short_lived_thread(4, 1_000_000);
        // parallel_pipeline(10, 4, 5, 200);
        pass_data(10, 100);
    }
    fn short_lived_thread(thread_num: usize, vec_size: usize) {
        let mut rng = rand::thread_rng();
        let mut arr = Vec::new();
        let start = Instant::now();
        for _ in 0..vec_size {
            arr.push(rng.gen::<i32>())
        }
        let duration = start.elapsed();
        println!("random {} numbers takes: {:?}", vec_size, duration);
        let start = Instant::now();
        let max = find_max(&arr, thread_num);
        let duration = start.elapsed();
        println!(
            "max is {:?}, use {} threads takes: {:?}",
            max, thread_num, duration
        );
        let start = Instant::now();
        let max = arr.iter().max();
        let duration = start.elapsed();
        println!("max is {:?}, use iter takes: {:?}", max, duration);
        let mut max = 0;
        let start = Instant::now();
        for num in arr {
            max = max.max(num);
        }
        let duration = start.elapsed();
        println!("max is {:?}, use for loop takes: {:?}", max, duration);
    }

    fn find_max(arr: &Vec<i32>, thread_num: usize) -> Option<i32> {
        if arr.len() <= thread_num {
            return arr.iter().cloned().max();
        }

        let size = arr.len() / thread_num;

        crossbeam::scope(|s| {
            let mut thread_vec = Vec::new();
            for i in 0..thread_num {
                let sub_arr = arr.iter().skip(i * size).take(size);
                thread_vec.push(s.spawn(move |_| sub_arr.max()));
            }
            let mut max_vec = Vec::new();
            for thread in thread_vec {
                max_vec.push(thread.join().unwrap()?);
            }
            match max_vec.iter().max() {
                Some(n) => Some(**n),
                None => None,
            }
        })
        .unwrap()
    }

    fn parallel_pipeline(msg_num: u64, thread_num: u8, cap: usize, sleep_millis: u64) {
        let (tx, rx) = bounded(cap);
        let (tx1, rx1) = bounded(cap);
        crossbeam::scope(|s| {
            s.spawn(|_| {
                for i in 0..msg_num {
                    println!("Source sent {}", i);
                    tx.send(i).unwrap();
                }
                drop(tx);
            });

            for _ in 0..thread_num {
                let (send, recive) = (tx1.clone(), rx.clone());
                s.spawn(move |_| {
                    std::thread::sleep(Duration::from_millis(sleep_millis));
                    for msg in recive.iter() {
                        println!("Worker {:?} received {}", std::thread::current().id(), msg);
                        send.send(msg * 2).unwrap();
                    }
                });
            }

            drop(tx1);

            for msg in rx1.iter() {
                println!("Sink received {}", msg);
            }
        })
        .unwrap()
    }

    fn pass_data(msg_num: u64, sleep_millis: u64) {
        let (snd, rcv) = unbounded();
        crossbeam::scope(|s| {
            s.spawn(|_| {
                for i in 0..msg_num {
                    snd.send(i).unwrap();
                    thread::sleep(Duration::from_millis(sleep_millis));
                }
            });
        })
        .unwrap();
        for _ in 0..msg_num {
            let msg = rcv.recv().unwrap();
            println!("Received {}", msg);
        }
    }
}
