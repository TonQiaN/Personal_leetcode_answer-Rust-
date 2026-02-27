struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if nums1[j - 1] == nums2[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }
}

fn main() {
    println!("Hello, world!");
}
