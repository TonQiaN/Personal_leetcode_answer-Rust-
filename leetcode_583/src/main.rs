struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let word1_bytes = word1.as_bytes();
        let word2_bytes = word2.as_bytes();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if word1_bytes[j - 1] == word2_bytes[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = i32::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1);
                }
            }
        }
        dp[m][n]
    }
}

fn main() {
    println!("Hello, world!");
}
