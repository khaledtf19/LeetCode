use std::collections::HashMap;

fn main() {
    group_anagrams(vec![
        "eat".into(),
        "tea".into(),
        "tan".into(),
        "ate".into(),
        "nat".into(),
        "bat".into(),
    ]);
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    for str in strs.iter() {
        let mut b = str.clone().into_bytes();
        b.sort();
        let s = String::from_utf8(b).unwrap();
        let v = hm.entry(s).or_insert(vec![]);
        v.push(str.to_string());
    }
    hm.into_values().collect::<Vec<Vec<String>>>()
}
