fn main() {
    length_of_last_word("hello   ".to_string());
}

pub fn length_of_last_word(s: String) -> i32 {
    s.trim_end().split(' ').rev().next().unwrap().len() as i32
}
