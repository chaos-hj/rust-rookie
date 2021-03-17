fn main() {
    let number_list = vec![34, 50, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 2, 43, 8];
    let mut largest = number_list[0];
    for &number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number in {:?} is {}", number_list, largest);

    let number_list = vec![99, 347, 190, 89];
    let mut largest = number_list[0];
    for number in &number_list {
        if *number > largest {
            largest = *number;
        }
    }
    println!("The largest number in {:?} is {}", number_list, largest);

    let number_list = vec![34, 55, 89];
    let result = largest_num(&number_list);
    println!("The largest number in {:?} is {}", number_list, result);

    let i32_list = vec![100, 200, 300];
    let result = largest_i32(&i32_list);
    println!("The largest i32 in {:?} is {}", i32_list, result);

    let char_list = vec!['a', '/', 'Y'];
    let result = largest_char(&char_list);
    println!("The largest char in {:?} is {}", char_list, result);
}

fn largest_num(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn _largest_t<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn _test_1() {
    let _point = Point { x: 4, y: 5 };
}

struct Point1<T, U> {
    x: T,
    y: U,
}

fn _test_2() {
    let _point = Point1 { x: 4, y: 5.0 };
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn _test_3() {
    let p = Point { x: 4, y: 5 };
    println!("p.x = {}", p.x());
}

impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn _test_4(p: &Point<f32>) {
    let _distance = p.distance();
}

impl<T: Copy, U> Point1<T, U> {
    fn mixup<V, W>(&self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn _test_5() {
    let p1 = Point1 { x: 3, y: 3.3 };
    let p2 = Point1 { x: true, y: 'l' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn title(&self) -> &str {
        "hello world"
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn title(&self) -> &str {
        &self.headline[..]
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("Exciting news! {}", item.summarize());
}

use std::fmt::Display;
use Clone;
pub fn test_6(_item: impl Summary + Display) {}

fn _test_7(_x: impl Summary + Display, _y: impl Summary + Clone) {}

fn _test_8<T, U>(_t: T, _u: U)
where
    T: Summary + Display,
    U: Summary + Clone,
{
}

fn _test_9<T, U>(_t: T, _u: U) -> i32
where
    T: Clone + Display,
    U: Summary,
{
    2
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: "Jankos".to_string(),
        content: "LOL".to_string(),
        reply: true,
        retweet: false,
    }
}

fn _switch(clazz_type: String) -> impl Summary {
    if clazz_type == "Tweet".to_string() {
        Tweet {
            username: "l".to_string(),
            content: "k".to_string(),
            reply: true,
            retweet: false,
        }
    } else {
        // NewsArticle {
        //     headline: "k".to_string(),
        //     location: "loca".to_string(),
        //     author: "lili".to_string(),
        //     content: "ko".to_string(),
        // }
        Tweet {
            username: "l".to_string(),
            content: "k".to_string(),
            reply: true,
            retweet: false,
        }
    }
}

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The bigger member is {}", self.x);
        } else {
            println!("The bigger member is {}", self.y);
        }
    }
}

fn _test_10() {
    let _r;
    {
        let x = 5;
        _r = &x;
        // x is dropped here
    }
    // println!("{}", r); // use borrowed x
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let len1 = s1.len();
    let len2 = s2.len();
    if len1 > len2 {
        s1
    } else {
        s2
    }
}

fn _test_11() {
    let s1 = String::from("hello");
    let s2 = "xyz";
    let result = longer(s1.as_str(), s2);
    println!("The longer string betwwen {} and {} is {}", s1, s2, result);
}

fn _test_12() {
    let s1 = String::from("long string is long");
    let _result1;
    {
        let s2 = String::from("xyz");
        let result = longer(s1.as_str(), s2.as_str());
        _result1 = longer(s1.as_str(), s2.as_str());
        println!("The longer string is {}", result);
    }
    // println!("{}", result1);
}

fn _test_13<'a>() -> &'a str {
    "hello"
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn _test_14() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

impl ImportantExcerpt<'_> {
    fn _level(&self) -> i32 {
        3
    }
}

impl ImportantExcerpt<'_> {
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn _test_15() {
    let _s = "hello";
    let _s: &'static str = "I have a static lifetime";
}

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
