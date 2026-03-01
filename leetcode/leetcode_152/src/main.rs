struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // (max, min)
        let mut dp = vec![[nums[0], nums[0]]; n];
        let mut ans = nums[0];
        for i in 1..n {
            dp[i][0] = nums[i].max(dp[i - 1][0] * nums[i]).max(dp[i - 1][1] * nums[i]);
            dp[i][1] = nums[i].min(dp[i - 1][0] * nums[i]).min(dp[i - 1][1] * nums[i]);
            ans = ans.max(dp[i][0]);
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
