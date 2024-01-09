use std::{collections::HashMap, rc::Rc};

fn main() {
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    let res = word_pattern(pattern, s);
    println!("{res}");
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let pattern = pattern.chars().collect::<Rc<[char]>>();
    if s.split(" ").collect::<Rc<[&str]>>().len() != pattern.len() {
        return false;
    }
    let mut hm: HashMap<char, &str> = HashMap::new();
    for (i, str) in s.split(" ").enumerate() {
        match hm.get(&pattern[i]) {
            Some(&v) => {
                if v != str {
                    return false;
                }
            }
            None => {
                if hm.values().collect::<Rc<[&&str]>>().contains(&&str) {
                    return false;
                } else {
                    hm.insert(pattern[i], str);
                }
            }
        }
    }

    true
}
