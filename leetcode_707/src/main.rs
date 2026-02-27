#[derive(Default)]
struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        MyLinkedList::default()
    }
    
    fn get(&self, index: i32) -> i32 {
        if index == 0 {
            return self.val;
        }
        let mut head = self.next.as_mut().unwrap();
        for i in 0..index {
            head = head.next.as_mut().unwrap();
        }
        head.val
        
    }
    
    fn add_at_head(&self, val: i32) {
        
    }
    
    fn add_at_tail(&self, val: i32) {
        
    }
    
    fn add_at_index(&self, index: i32, val: i32) {
        
    }
    
    fn delete_at_index(&self, index: i32) {
        
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

fn main() {
    println!("Hello, world!");
}
