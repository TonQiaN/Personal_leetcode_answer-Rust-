struct Solution {}

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut best = 0;
        let mut dp = vec![0; m + 1];
        for i in (0..n).rev() {
            for j in 0..m {
                if nums1[i] == nums2[j] {
                    dp[j] = dp[j + 1] + 1;
                    best = best.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
            }
        }
        best
    }
}
// impl Solution {
//     pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
//         let n = nums1.len();
//         let m = nums2.len();
//         let mut best = 0;
//         let mut dp = vec![vec![0; n + 1]; m + 1];
//         for i in 1..m + 1 {
//             for j in 1..n + 1 {
//                 if nums1[j - 1] == nums2[i - 1] {
//                     dp[i][j] = dp[i - 1][j - 1] + 1;
//                     best = best.max(dp[i][j]);
//                 }
//             }
//         }
//         best
//     }
// }

fn main() {
    println!("Hello, world!");
}
