struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let (mut i, mut j) = (0, 0);
        if s_bytes.len() == 0 {
            return true;
        }
        if t_bytes.len() == 0 {
            return false;
        }
        loop {
            if s_bytes[i] == t_bytes[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
            if i == s_bytes.len() || j == t_bytes.len() {
                break;
            }
        }
        i == s_bytes.len()
    }
}
fn main() {
    println!("Hello, world!");
}
