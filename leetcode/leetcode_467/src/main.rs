struct Solution {}

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let mut dp = vec![0; 26];
        let s_bytes = s.as_bytes();
        let mut next = s_bytes[0] - b'a';
        let mut len = 0;
        for i in s_bytes {
            let curr = i - b'a';
            if next == curr {
                len += 1;
            } else {
                len = 1;
            }
            dp[curr as usize] = dp[curr as usize].max(len);
            next = (curr + 1) % 26;
        }
        dp.iter().sum()
    }
}

fn main() {
    println!("Hello, world!");
}
