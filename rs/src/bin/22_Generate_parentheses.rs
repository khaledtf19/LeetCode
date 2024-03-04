fn main() {
    let res = generate_parenthesis(3);
    dbg!(res);
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    dfs(0, 0, "".to_string(), &mut res, n);
    res
}

fn dfs(left: i32, right: i32, s: String, res: &mut Vec<String>, n: i32) {
    if s.len() == n as usize * 2 {
        res.push(s);
        return;
    }

    if left < n {
        dfs(left + 1, right, s.clone() + "(", res, n)
    }

    if right < left {
        dfs(left, right + 1, s.clone() + ")", res, n)
    }
}
