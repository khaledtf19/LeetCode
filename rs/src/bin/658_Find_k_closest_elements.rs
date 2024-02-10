fn main() {
    let res = find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, 11);
    println!("{res:?}");
}

pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let (mut lo, mut hi) = (0, arr.len() - k as usize);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        if x - arr[mid] > arr[mid + k as usize] - x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    arr[lo..(lo + k as usize)].into()
}
