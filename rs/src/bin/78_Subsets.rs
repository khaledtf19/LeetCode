fn main() {
    let res = subsets(vec![1, 2, 3]);
    println!("{:?}", res);
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![]];
    recurse(&mut res, &nums, &mut vec![], 0);
    res
}

pub fn recurse(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, curr: &mut Vec<i32>, idx: usize) {
    if idx >= nums.len() {
        return;
    }
    curr.push(nums[idx]);
    res.push(curr.to_vec());
    dbg!(curr.clone());
    recurse(res, nums, curr, idx + 1);
    dbg!(curr.clone());
    curr.pop();
    recurse(res, nums, curr, idx + 1)
}
