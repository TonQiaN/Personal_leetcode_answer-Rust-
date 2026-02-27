struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut prev = ' ';
        let mut max = 1;
        let mut temp_max = 1;
        for c in s.chars() {
            if prev == c {
                temp_max += 1;
                max = max.max(temp_max);
            } else {
                temp_max = 1;
                prev = c;
            }
        }
        max
    }
}

fn main() {
    Solution::max_power("leetcode".to_string());
}
