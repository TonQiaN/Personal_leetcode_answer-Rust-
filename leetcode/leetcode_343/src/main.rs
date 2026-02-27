struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 0;
        dp[2] = 1;
        for i in 3..n + 1 {
            for j in 1..i {
                let max = i32::max(j as i32 * dp[i - j], (j * (i - j)) as i32);
                dp[i] = dp[i].max(max);
            }
        }
        dp[n]
    }
}

fn main() {
    println!("Hello, world!");
}
