fn main() {
    cal_points(vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string(),
    ]);
}

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for s in operations.iter() {
        let len = stack.len();
        match s.as_str() {
            "+" => stack.push(stack[len - 1] + stack[len - 2]),
            "D" => stack.push(stack[stack.len() - 1] * 2),
            "C" => {
                stack.pop();
            }
            x => stack.push(x.parse().unwrap()),
        }
    }
    stack.iter().sum()
}
