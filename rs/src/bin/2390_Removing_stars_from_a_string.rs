fn main() {
    let res = remove_stars("erase*****".to_string());
    dbg!(res);
}

pub fn remove_stars(s: String) -> String {
    let mut stack: String = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '*' => {
                stack.pop();
            }
            _ => stack.push(ch),
        }
    }

    stack
}
