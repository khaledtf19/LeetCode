fn main() {
    unimplemented!();
}

pub fn guess(num: i32) -> i32 {
    let target = 6;
    match num.cmp(&target) {
        std::cmp::Ordering::Less => return 1,
        std::cmp::Ordering::Greater => return -1,
        std::cmp::Ordering::Equal => return 0,
    }
}

unsafe fn gussNumber(n: i32) -> i32 {
    let (mut lo, mut hi) = (0, n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match guess(mid) {
            -1 => hi = mid,
            1 => lo = mid + 1,
            0 => return mid,
            _ => {}
        }
    }
    0
}
