struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (n, m) = (text1.len(), text2.len());
        let (text1_bytes, text2_bytes) = (text1.as_bytes(), text2.as_bytes());
        let mut dp = vec![0; m + 1];
        for i in 1..n + 1 {
            let mut left_up = 0;
            for j in 1..m + 1 {
                let backup = dp[j];
                if text1_bytes[i - 1] == text2_bytes[j - 1] {
                    dp[j] = left_up + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                left_up = backup;
            }
        }
        dp[m]
    }
}
// impl Solution {
//     pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
//         let (n, m) = (text1.len(), text2.len());
//         let (text1_bytes, text2_bytes) = (text1.as_bytes(), text2.as_bytes());
//         let mut dp = vec![vec![0; m + 1]; n + 1];
//         for i in 1..n + 1 {
//             for j in 1..m + 1 {
//                 if text1_bytes[i - 1] == text2_bytes[j - 1] {
//                     dp[i][j] = dp[i - 1][j - 1] + 1;
//                 } else {
//                     dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
//                 }
//             }
//         }
//         dp[n][m]
//     }
// }

fn main() {
    println!("Hello, world!");
}
