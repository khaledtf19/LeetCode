fn main() {
    let res = guessNumber(2);
    dbg!(res);
}
fn guess(num: i32) -> i32 {
    match num.cmp(&2) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

fn guessNumber(n: i32) -> i32 {
    if guess(n) == 0 {
        return n;
    }
    let (mut start, mut end) = (0, n);
    let mut mid = 0;
    while start < end {
        mid = start + (end - start) / 2;
        dbg!(guess(mid));
        dbg!(mid);
        match guess(mid) {
            1 => start = mid + 1,
            -1 => end = mid,
            0 => return mid,
            _ => return 0,
        }
    }
    mid
}
