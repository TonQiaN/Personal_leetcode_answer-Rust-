struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        if s_bytes.len() == 0 {
            return 0;
        }
        let mut ans = 0;
        let mut dp = vec![0; s_bytes.len()];
        for i in 1..s_bytes.len() {
            if s_bytes[i] == b'(' {
                continue;
            } else {
                let prev = (i - dp[i - 1] - 1) as i32;
                if prev < 0 {
                    continue;
                }
                if s_bytes[prev as usize] == b')' {
                    continue;
                } else {
                    dp[i] = 2 + dp[i - 1] + if prev != 0 { dp[prev as usize - 1] } else { 0 };
                    ans = ans.max(dp[i]);
                }
            }
        }
        ans as i32
    }
}

fn main() {
    println!("Hello, world!");
}
