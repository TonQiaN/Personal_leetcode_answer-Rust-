struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut char_last_index: HashMap<char, i32> = HashMap::new();
        let (mut left, mut max) = (0, 0);
        for (c, i) in s.chars().zip(0..) {
            if let Some(old) = char_last_index.get_mut(&c) {
                left = left.max(*old + 1);
                *old = i;
            } else {
                char_last_index.insert(c, i);
            }
            max = i32::max(max, i - left + 1);
        }
        max
    }
}
fn main() {
    println!("Hello, world!");

}
