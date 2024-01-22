fn main() {
    let mut v = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut v);
    println!("{:?}", v);
}

pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len / 2) {
        let k = len - 1 - i;
        let tmp = s[i];
        s[i] = s[k];
        s[k] = tmp;
    }
}
