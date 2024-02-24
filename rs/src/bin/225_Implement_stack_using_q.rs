fn main() {
    unimplemented!();
}

struct MyStack {
    head: Option<i32>,
    tail: Option<i32>,
    stack: Vec<i32>,
    length: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            stack: Vec::new(),
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        self.tail = Some(x);
        if self.length == 0 {
            self.head = Some(x)
        }
        self.length += 1
    }

    fn pop(&mut self) -> i32 {
        let res = self.stack.pop();
        self.length -= 1;
        if self.length == 0 {
            self.tail = None;
            self.head = None;
        } else {
            self.tail = Some(self.stack[self.length - 1])
        }
        res.unwrap()
    }

    fn top(&self) -> i32 {
        self.tail.unwrap()
    }

    fn empty(&self) -> bool {
        self.length == 0
    }
}
