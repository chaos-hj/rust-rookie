#[allow(dead_code)]
pub fn it_works() {
    println!("Find the sum of all the squared odd numbers under 1000");

    let res: u32 = (1..).map(|x| x * x)
    .take_while(|&n| n< 1000)
    .filter(|n| n % 2 == 1)
    .fold(0, |sum, i|  sum +i);

    println!("{}", res);
}


