fn main() {
    unimplemented!()
}

pub fn evel_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for ch in tokens.iter() {
        match ch.as_str() {
            "+" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x + y);
            }
            "-" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(-x + y);
            }
            "*" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x * y);
            }
            "/" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push((y / x) | 0);
            }
            _ => stack.push(ch.parse().unwrap()),
        }
    }
    stack[0]
}
