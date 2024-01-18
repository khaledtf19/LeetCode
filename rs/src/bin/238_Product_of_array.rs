fn main() {
    let res = product_except_self(vec![1, 2, 3, 4]);
    println!("{:?}", res);
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = vec![1; nums.len()];
    let mut prefix = 1;
    let mut postfix = 1;

    for i in 0..nums.len() {
        arr[i] = prefix;
        prefix *= nums[i]
    }
    for i in (0..nums.len()).rev() {
        arr[i] *= postfix;
        postfix *= nums[i]
    }

    arr
}
