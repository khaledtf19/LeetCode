fn main() {
    let res = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{:?}", res)
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut max, mut l, mut r) = (0, 0, height.len() - 1);
    while l < r {
        let (curr_l, curr_r) = (height[l], height[r]);
        let curr = curr_l.min(curr_r) * (r - l) as i32;
        max = curr.max(max);
        if curr_l < curr_r {
            l += 1;
        } else {
            r -= 1;
        }
    }
    return max;
}
