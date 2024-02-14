fn main() {
    num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4);
}
pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut sum = 0;
    let mut res = 0;
    if arr.len() < k as usize {
        return 0;
    }
    for i in 0..k {
        sum += arr[i as usize];
    }
    if sum / k >= threshold {
        res += 1;
    }
    for (i, num) in arr.iter().enumerate().skip(k as usize) {
        sum -= arr[i - k as usize];
        sum += num;
        if sum / k >= threshold {
            res += 1;
        }
    }
    dbg!(res);

    res
}
