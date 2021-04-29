use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, NormalError};

#[allow(unused)]
pub fn it_works() {
    random_num();
    rand_in_range();
    rand_distribution();
    rand_tuple();
    rand_alphanum();
    rand_distr();
    rand_password();
}

fn random_num() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random f64: {}", rng.gen::<f64>());
}

fn rand_in_range() {
    let mut rng = rand::thread_rng();
    println!("Intger: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..=10.0));
}

fn rand_distribution() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn rand_distr() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let mut i = 0;
    println!("from a N(2, 9) distribution");
    while i < 10 {
        let v = normal.sample(&mut rng);
        println!("{}", v);
        i += 1;
    }
    println!("from a N(2, 9) distribution");
    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

fn rand_tuple() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, i32, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random Tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

fn rand_alphanum() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}

fn rand_password() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_SIZE: u8 = 30;

    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_SIZE)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}", password);
}
