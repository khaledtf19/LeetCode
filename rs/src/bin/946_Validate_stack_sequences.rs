fn main() {
    let res = validate_stack_sequences(vec![2, 1, 0], vec![1, 2, 0]);
    dbg!(res);
}

pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut p_idx = 0;
    for (i, &num) in pushed.iter().enumerate() {
        if pushed[i] == popped[p_idx] {
            p_idx += 1;
        } else {
            while !stack.is_empty() && stack.last().unwrap() == &popped[p_idx] {
                stack.pop();

                p_idx += 1;
            }
            stack.push(num)
        }
    }
    dbg!(stack.clone());
    while p_idx < popped.len() {
        let expected_pop = popped[p_idx];
        if let Some(new_pop) = stack.pop() {
            dbg!(expected_pop, new_pop);
            if expected_pop != new_pop {
                dbg!("hi");
                return false;
            }
        } else {
            return false;
        }
        p_idx += 1;
    }

    stack.is_empty()
}
