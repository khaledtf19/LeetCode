struct MinStack {
    stack: Vec<i32>,
    tail: Option<i32>,
    min: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            tail: None,
            min: None,
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.tail = Some(val);

        if let Some(old_min) = self.min {
            self.min = Some(val.min(old_min))
        } else {
            self.min = Some(val)
        }
    }

    fn pop(&mut self) {
        let out = self.stack.pop();
        if self.stack.len() == 0 {
            self.tail = None;
            self.min = None;
            return;
        }
        self.tail = Some(self.stack[self.stack.len() - 1]);
        if out.unwrap() == self.min.unwrap() {
            let mut curr_min = self.stack[0];
            for &num in self.stack.iter() {
                curr_min = num.min(curr_min);
            }
            self.min = Some(curr_min);
        }
    }

    fn top(&self) -> i32 {
        self.tail.unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min.unwrap()
    }
}
