struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut count = 0;
        let mut dp = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if s_bytes[i] == s_bytes[j] {
                    if j - i <= 1 || dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");
}
