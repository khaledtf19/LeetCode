fn main() {
    let s = String::from("011101");
    println!("------\noutput: {}\n------", max_score(s));
}
pub fn max_score(s: String) -> i32 {
    let mut res = 0;
    for (i, _) in s.split("").enumerate() {
        if i as i32 - 1 >= 1 && i - 1 < s.len() {
            let (left, right) = s.split_at(i - 1);
            let left = left.chars().filter(|ch| ch == &'0').count() as i32;
            let right = right.chars().filter(|ch| ch == &'1').count() as i32;
            if res < left + right {
                res = left + right
            }
        }
    }
    res
}
