fn main() {
    let res = missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
    println!("{res}");
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    return (nums.len() * (1 + nums.len()) / 2) as i32
        - nums.iter().fold(0, |state, num| num + state);
}
