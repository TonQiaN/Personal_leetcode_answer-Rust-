#[derive(Default)]
struct MinStack {
    data_stack: Vec<i32>,
    min_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self::default()
    }
    
    fn push(&mut self, val: i32) {
        self.data_stack.push(val);
        if let Some(&min) = self.min_stack.last() {
            if val <= min {
                self.min_stack.push(val);
            }
        } else {
            self.min_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        if let Some(last) = self.data_stack.pop() {
            if last == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        *self.data_stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

 fn main () {}