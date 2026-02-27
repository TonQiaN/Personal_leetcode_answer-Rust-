struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for end in 0..=n {
            for word in &word_dict {
                let word_len = word.len();
                if word_len > end {
                    continue;
                }
                if dp[end - word_len] && &s[end - word_len..end] == word {
                    dp[end] = true;
                }
            }
        }
        dp[n]
    }
}

fn main() {
    println!("Hello, world!");
}
