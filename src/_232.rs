#[derive(Default)]
pub struct MyQueue {
    stack: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn push(&mut self, x: i32) {
        let mut tmp_stack: Vec<i32> = Vec::new();
        while !self.stack.is_empty() {
            tmp_stack.push(self.stack.pop().unwrap());
        }
        self.stack.push(x);
        while !tmp_stack.is_empty() {
            self.stack.push(tmp_stack.pop().unwrap());
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }
    pub fn peek(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
