struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return i32::max(nums[0], nums[1]);
        }
        
        let mut dp1 = vec![0; n - 1];
        dp1[0] = nums[0];
        dp1[1] = i32::max(nums[0], nums[1]);
        for i in 2..n - 1 {
            dp1[i] = i32::max(dp1[i - 1], dp1[i - 2] + nums[i]);
        }

        let mut dp2 = vec![0; n - 1];
        dp2[0] = nums[1];
        dp2[1] = i32::max(nums[1], nums[2]);
        for i in 3..n {
            dp2[i - 1] = i32::max(dp2[i - 2], dp2[i - 3] + nums[i]);
        }

        i32::max(dp1[n - 2], dp2[n - 2])
    }
}

fn main() {
    println!("Hello, world!");
}
