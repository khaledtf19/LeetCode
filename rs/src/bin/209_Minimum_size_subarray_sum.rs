fn main() {
    let res = min_sub_array_len(6, vec![10, 2, 3]);
    dbg!(res);
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut start, mut end, mut res) = (0, 1, 0);

    if nums.len() < 2 {
        if nums.len() == 0 {
            return 0;
        }
        if nums[start] >= target {
            return 1;
        } else {
            return 0;
        }
    }
    if nums[start] >= target || nums[end] >= target {
        return 1;
    }
    let mut sum = nums[start] + nums[end];

    while start < end {
        if nums[end] == target {
            return 1;
        }
        if sum >= target {
            if res == 0 || res > end + 1 - start {
                res = end + 1 - start
            }
            sum -= nums[start];
            start += 1;
        } else if sum <= target {
            if end + 1 >= nums.len() {
                break;
            }
            end += 1;
            sum += nums[end];
        }
    }
    res as i32
}
