fn main() {
    remove_duplicates(&mut vec![1, 1, 2]);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut j = 0;
    let mut count = 0;
    for i in 1..nums.len() {
        if nums[j] != nums[i] {
            count = 0;
            j += 1;
            nums[j] = nums[i];
        } else {
            count += 1;
            if count < 2 {
                j += 1;
                nums[j] = nums[i];
            }
        }
    }
    (j + 1) as i32
}
