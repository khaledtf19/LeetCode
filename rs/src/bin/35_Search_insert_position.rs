fn main() {
    unimplemented!();
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return mid as i32,
        }
    }
    if nums[lo] < target {
        lo += 1;
    }

    lo as i32
}
