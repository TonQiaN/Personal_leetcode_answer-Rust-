struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut dp = vec![0; n];
        for i in (0..n).rev() {
            let mut left_down = 1;
            for j in i..n {
                let backup = dp[j];
                if s_bytes[i] == s_bytes[j] {
                    if j - i <= 1 {
                        dp[j] = j - i + 1;
                    } else {
                        dp[j] = left_down + 2;
                    }
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                left_down = backup;
            }
        }
        dp[n - 1] as i32
    }
}

// impl Solution {
//     pub fn longest_palindrome_subseq(s: String) -> i32 {
//         let n = s.len();
//         let s_bytes = s.as_bytes();
//         let mut dp = vec![vec![0; n]; n];
//         let mut best = 0;
//         for i in (0..n).rev() {
//             for j in i..n {
//                 if s_bytes[i] == s_bytes[j] {
//                     if j - i <= 1 {
//                         dp[i][j] = j - i + 1;
//                     } else {
//                         dp[i][j] = dp[i + 1][j - 1] + 2;
//                     }
//                 } else {
//                     dp[i][j] = usize::max(dp[i + 1][j], dp[i][j - 1]);
//                 }
//                 best = best.max(dp[i][j]);
//             }
//         }
//         best as i32
//     }
// }

fn main() {
    println!("Hello, world!");
}
