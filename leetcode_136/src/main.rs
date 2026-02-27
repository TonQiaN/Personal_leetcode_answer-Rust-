struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        for i in nums {
            x ^= i;
        }
        x
    }
}

fn main() {}