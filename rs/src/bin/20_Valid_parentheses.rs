use std::collections::HashMap;

fn main() {
    is_valid("[".to_string());
}

struct Stack {}

pub fn is_valid(s: String) -> bool {
    let hm = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
    let mut stack: Vec<char> = vec![];
    for ch in s.chars() {
        match hm.get(&ch) {
            Some(v) => {
                if let Some(k) = stack.pop() {
                    if v != &k {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => stack.push(ch),
        }
    }
    stack.is_empty()
}
