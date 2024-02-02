fn main() {
    find_duplicate(vec![1, 3, 4, 2, 2]);
}

pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let index: usize = i32::abs(nums[i]) as usize;
        if nums[index - 1] < 0 {
            return i32::abs(nums[i]);
        } else {
            nums[index - 1] *= -1;
        }
    }
    0
}
