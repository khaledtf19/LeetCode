fn main() {
    let res = two_sum(vec![2, 3, 4], 6);
    println!("{:?}", res);
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r) = (0, numbers.len() - 1);
    while l < r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            std::cmp::Ordering::Less => l += 1,
            std::cmp::Ordering::Greater => r -= 1,
            std::cmp::Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
        }
    }
    vec![]
}
