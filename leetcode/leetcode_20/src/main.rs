struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut my_stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                my_stack.push(c);
            } else {
                match my_stack.pop() {
                    None => return false,
                    Some(last) => {
                        match (last, c) {
                            ('(', ')') => continue,
                            ('[', ']') => continue,
                            ('{', '}') => continue,
                            (_, _) => return false
                        }
                    }
                }
            }
        }
        my_stack.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
