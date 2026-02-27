struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut fib_map = HashMap::new();
        fib_map.insert(0, 0);
        fib_map.insert(1, 1);
        fn inner_fb(n:i32, fib_map: &mut HashMap<i32, i32>) -> i32 {
            if let Some(value) = fib_map.get(&n) {
                return *value;
            } else {
                let val = inner_fb(n - 2, fib_map) + inner_fb(n - 1, fib_map);
                fib_map.insert(n, val);
                return val;
            }
        }
        inner_fb(n, &mut fib_map)
    }
}

fn main() {
    println!("Hello, world!");
}
