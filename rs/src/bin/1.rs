fn main() {
    let res = is_acronym(
        vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ],
        "abc".to_string(),
    );
    println!("{}", res);
}

pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    if words.len() != s.len() {
        return false;
    }
    for (i, word) in words.iter().enumerate() {
        if word.as_bytes()[0] != s.as_bytes()[i] {
            return false;
        }
    }
    true
}
