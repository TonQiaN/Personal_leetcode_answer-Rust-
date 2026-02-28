struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut ans = String::new();
        let s_bytes = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let (mut best_l, mut best_r) = (0usize, 0usize);
        for i in (0..n).rev() {
            dp[i][i] = true;
            for j in i + 1..n {
                if (s_bytes[i] == s_bytes[j]) && (j - i == 1 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    if best_r - best_l < j - i {
                        (best_l, best_r) = (i, j);
                    }
                }
            }
        }
        s[best_l..=best_r].to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
