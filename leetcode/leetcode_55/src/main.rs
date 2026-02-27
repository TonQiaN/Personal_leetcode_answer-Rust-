struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut end = nums[0] as usize;
        for i in 0..nums.len() {
            if i > end {
                return false;
            }
            end = end.max(i + nums[i] as usize);
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
