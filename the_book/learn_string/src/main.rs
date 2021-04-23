fn main() {

    let mut _s = String::new();

    let data = "initial contents";

    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let s2 = "hello";
    s.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let mut _s3 = s1 + &s2;

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = t1 + "-" + &t2 + "-" +&t3;
    let _t = format!("{}-{}-{}", t, t2, t3);

    let str = "नमस्ते".to_string();
    println!("{} length is {}", str, str.len());

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{} 0-4 byte is {}", hello, s);

    println!("for each char in [नमस्ते]");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("for each byte in [नमस्ते]");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
