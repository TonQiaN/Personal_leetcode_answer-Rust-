struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        // let mut s_bytes = s.into_bytes();
        let mut s_bytes = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < s_bytes.len() {
            if i + k as usize <= s_bytes.len() {
                s_bytes[i..i + k as usize].reverse();
            } else {
                s_bytes[i..].reverse();
            }
            i += 2 * k as usize;
        }
        // String::from_utf8(s_bytes).unwrap()
        s_bytes.into_iter().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
