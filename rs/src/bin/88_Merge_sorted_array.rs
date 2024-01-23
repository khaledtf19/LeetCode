fn main() {
    merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut left = nums1[0..(m as _)].to_vec();
    let right = nums2;
    let mut i = 0;

    loop {
        match (left.first(), right.first()) {
            (None, None) => {
                break;
            }
            (None, Some(_)) => nums1[i] = right.remove(0),
            (Some(_), None) => nums1[i] = left.remove(0),
            (Some(a), Some(b)) => {
                let next = if b < a {
                    right.remove(0)
                } else {
                    left.remove(0)
                };
                nums1[i] = next
            }
        }
        i += 1;
    }
    println!("{:?}", nums1);
}
