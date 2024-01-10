fn main() {
    let res = majority_element(vec![3, 2, 3]);
    println!("{:?}", res);
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0i32;
    let mut curr_num = 0i32;
    for num in nums {
        if count == 0 {
            curr_num = num;
            count = 1;
        } else {
            count += if num == curr_num { 1 } else { -1 }
        }
    }
    curr_num
}
