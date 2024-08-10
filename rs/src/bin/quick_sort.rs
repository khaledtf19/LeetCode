fn main() {
    let res = sort_array(vec![10, 5, 20, 14, 3, 1]);
    dbg!(res);
}

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    sort(&mut nums);
    nums
}

pub fn partition(arr: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
    let pivot = arr[hi as usize];
    let mut idx: i32 = lo - 1;

    for i in lo..hi {
        if pivot > arr[i as usize] {
            idx += 1;
            let tmp = arr[idx as usize];
            arr[idx as usize] = arr[i as usize];
            arr[i as usize] = tmp;
        }
    }
    idx += 1;

    arr[hi as usize] = arr[idx as usize];
    arr[idx as usize] = pivot;
    idx
}

pub fn qs(arr: &mut Vec<i32>, lo: i32, hi: i32) {
    if lo >= hi {
        return;
    }
    let pivot = partition(arr, lo, hi);

    qs(arr, pivot + 1, hi);
    qs(arr, lo, pivot - 1);
}

pub fn sort(arr: &mut Vec<i32>) {
    qs(arr, 0, arr.len() as i32 - 1)
}
