struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..n {
            dp[0][i] = 1;
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if s_bytes[j - 1] == t_bytes[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
        dp[m][n]
    }
}

fn main() {
    println!("Hello, world!");
}
