use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            let v = self.queue.pop_front().unwrap();
            self.queue.push_back(v);
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    pub fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        assert!(!s.empty());
        assert_eq!(s.top(), 2);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 1);
        assert!(s.empty());
    }
}
