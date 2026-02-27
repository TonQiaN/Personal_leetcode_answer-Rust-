struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX - 1; n + 1];
        dp[0] = 0;

        let mut nums = vec![];
        for i in 1..=n {
            if i * i <= n {
                nums.push(i * i);
            } else {
                break
            }
        }

        for num in nums {
            for w in num..=n {
                dp[w] = dp[w].min(dp[w - num] + 1);
            }
        }

        dp[n]
    }
}
fn main() {
    println!("Hello, world!");
}
