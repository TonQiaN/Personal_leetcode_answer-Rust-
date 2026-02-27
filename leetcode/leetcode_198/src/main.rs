struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..n {
            dp[i] = i32::max(dp[i - 1], nums[i] + dp[i - 2]);
        }
        dp[n - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
