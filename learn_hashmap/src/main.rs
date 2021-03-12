use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);
    let key = "Blue";
    let socre = scores.get(&key);
    scores.entry(key).or_insert(10);

    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&filed_name, &filed_value);
    map.entry(&"key".to_string()).or_insert(&"Red".to_string());

    let teams = vec!["Blue", "Red"];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    scores.get(&"Yellow");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("[{}] word count {:?}", text, map);
}
