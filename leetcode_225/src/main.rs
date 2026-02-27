use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    queue: VecDeque<i32>,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack::default()
    }

    fn push(&mut self, x: i32) {
        self.count += 1;
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        for _ in 0..self.count - 1 {
            let top_number = self.queue.pop_front().unwrap();
            self.queue.push_back(top_number);
        }
        self.count -= 1;
        self.queue.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        for _ in 0..self.count - 1 {
            let top_number = self.queue.pop_front().unwrap();
            self.queue.push_back(top_number);
        }
        let number = self.queue.pop_front().unwrap();
        self.queue.push_back(number);
        number
    }

    fn empty(&self) -> bool {
        self.count == 0
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    println!("Hello, world!");
}
