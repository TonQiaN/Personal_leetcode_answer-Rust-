use std::i32;

struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut ans = usize::MAX;
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                ans = ans.min(i - start + 1);
                sum -= nums[start];
                start += 1;
            }
        }
        if ans == usize::MAX {
            return 0;
        }
        ans as i32
    }
}

fn main() {
    println!("Hello, world!");
}
