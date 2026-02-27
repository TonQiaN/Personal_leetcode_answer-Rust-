struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut next = vec![0; s.len()];
        let mut j = 0;
        let s_bytes = s.into_bytes();
        let n = s_bytes.len();
        for i in 1..n {
            while j > 0 && s_bytes[j] != s_bytes[i] {
                j = next[j - 1];
            }
            if s_bytes[i] == s_bytes[j] {
                j += 1;
            }
            next[i] = j;
        }
        
        n % (n - next[n - 1]) == 0 && next[n - 1] > 0
        // s.repeat(2)[1..2 * s.len() - 1].contains(&s)
    }
}

fn main() {
    println!("Hello, world!");
}
