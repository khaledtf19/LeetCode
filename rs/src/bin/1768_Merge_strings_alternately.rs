fn main() {
    merge_alternately("abcd".to_string(), "pq".to_string());
}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let len = word1.len().max(word2.len());
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut res = String::with_capacity(word1.len() + word2.len());

    for i in 0..len {
        if word1.len() > i {
            res.push(word1[i] as char)
        }
        if word2.len() > i {
            res.push(word2[i] as char)
        }
    }
    println!("{:?}", res);
    res
}
