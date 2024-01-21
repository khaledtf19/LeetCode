fn main() {
    is_palindrome("A man, a plan, a canal: Panama".into());
}

pub fn is_palindrome(s: String) -> bool {
    let word = s.chars().filter_map(|ch| {
        if ch.is_ascii_alphanumeric() {
            Some(ch.to_ascii_lowercase())
        } else {
            None
        }
    });
    let rev = word.clone().rev();
    word.take(s.len() / 2).eq(rev.take(s.len() / 2))
}
