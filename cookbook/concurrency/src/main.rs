fn main() {
    // crossbeam::it_works();
    // match global_state::it_works() {
    //     Ok(()) => println!("sucessful"),
    //     Err(res) => println!("{}", res),
    // }

    draw::it_works();
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

mod global_state {
    #![allow(unused)]
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
    }

    fn insert(fruit: &str) -> Result<(), &str> {
        let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
        db.push(fruit.to_string());
        Ok(())
    }

    pub fn it_works() -> Result<(), &'static str> {
        insert("apple")?;
        insert("orange")?;
        insert("peach")?;
        {
            let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
            db.iter()
                .enumerate()
                .for_each(|(index, item)| println!("{}: {}", index, item));
        }
        insert("graph")?;
        Ok(())
    }
}

mod draw {
    #![allow(unused)]
    use image::{ImageBuffer, Pixel, Rgb};
    use num::complex::Complex;
    use std::sync::mpsc::{channel, RecvError};
    use threadpool::ThreadPool;

    pub fn it_works() {
        let (width, height) = (1920, 1080);
        let mut img = ImageBuffer::new(width, height);
        let iterations = 300;

        let c = Complex::new(-0.8, 0.156);
        let (tx, rx) = channel();
        let pool = ThreadPool::new(num_cpus::get());

        for y in 0..height {
            let tx = tx.clone();
            pool.execute(move || {
                for x in 0..width {
                    // let i = julia(c, x, y, width, height, iterations);
                    // let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                    let inner_y = y as f32;
                    let inner_x = x as f32;
                    let inner_height = height as f32;
                    let inner_width = width as f32;

                    let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
                    let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

                    let mut i = iterations;

                    while zx * zx + zy * zy < 4.0 && i > 1 {
                        let tmp = zx * zx - zy * zy + c.re;
                        zy = 2.0 * zx * zy + c.im;
                        zx = tmp;
                        i -= 1;
                    }

                    // guesswork to make the rgb color values look okay
                    let r = (i << 3) as u8;
                    let g = (i << 5) as u8;
                    let b = (i * 4) as u8;
                    let pixel = Rgb::from_channels(r, g, b, 0);
                    tx.send((x, y, pixel));
                }
            });
        }

        for _ in 0..(width * height) {
            let (x, y, pixel) = rx.recv().unwrap();
            img.put_pixel(x, y, pixel);
        }

        img.save("output.png");
    }
}
