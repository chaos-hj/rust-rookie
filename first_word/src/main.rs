fn main() {
    let s = String::from("Hello world!");
    let len = first_blank_index(s);
    let s = String::from("Hi cargo!");
    let len = first_blank_index2(s);
    let s = String::from("Hallo rust!");
    let firstWord = first_word(&s);
}


fn first_blank_index(s: String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_blank_index2(s: String) -> usize {
    let bytes = s.as_bytes();
    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return i;
        }
    }
    return s.len();
}

// slice 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}