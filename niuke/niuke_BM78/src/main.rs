use core::num;
use std::i32;

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    pub fn rob(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        if n == 1 {
            return dp[0];
        }
        dp[1] = i32::max(nums[0], nums[1]);
        if n == 2 {
            return dp[1];
        }
        for i in 2..n {
            dp[i] = i32::max(dp[i - 1], dp[i - 2] + nums[i]);
        }
        dp[n - 1]
    }
}
