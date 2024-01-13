fn main() {
    let arr = find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e');
    println!("{:?}", arr);
}

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    for (i, word) in words.iter().enumerate() {
        match word.contains(x) {
            true => v.push(i as i32),
            false => {}
        }
    }
    v
}
