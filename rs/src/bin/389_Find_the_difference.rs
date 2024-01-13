fn main() {
    let char = find_the_difference("abcd".to_string(), "abcde".to_string());
    println!("{}", char);
}

pub fn find_the_difference(s: String, t: String) -> char {
    let mut count = 0u32;
    for ch in t.chars() {
        count += ch as u32;
    }
    for ch in s.chars() {
        count -= ch as u32
    }

    char::from_u32(count).unwrap()
}
