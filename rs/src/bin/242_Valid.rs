use std::collections::HashMap;

fn main() {
    let res = is_anagram("hello".to_string(), "helol".to_string());
    println!("{res}");
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1
    }
    for ch in t.chars() {
        *map.entry(ch).or_insert(0) -= 1
    }
    map.values().all(|v| *v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_true() {
        assert_eq!(is_anagram("hello".to_string(), "helol".to_string()), true)
    }

    #[test]
    fn test_valid_false() {
        assert_eq!(is_anagram("hello".to_string(), "helao".to_string()), false)
    }
}
