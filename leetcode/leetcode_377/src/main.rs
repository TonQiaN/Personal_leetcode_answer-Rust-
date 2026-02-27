struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for w in 0..=target {
            for &num in &nums {
                let num = num as usize;
                if w >= num {
                    dp[w] += dp[w - num];
                }
            }
        }
        dp[target]
    }
}

fn main() {
    println!("Hello, world!");
}
