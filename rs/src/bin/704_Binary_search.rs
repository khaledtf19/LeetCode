fn main() {
    let res = search(vec![6], 6);
    println!("{}", res);
}

pub fn search(numbers: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = numbers.len();
    while start < end {
        let mid = start + (end - start) / 2;
        match numbers[mid].cmp(&target) {
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Greater => end = mid,
            std::cmp::Ordering::Equal => return mid as i32,
        }
    }
    -1
}
