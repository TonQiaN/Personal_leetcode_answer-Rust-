struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut end = nums[0] as usize;
        let mut count = 1;
        let mut jump_max = 0;
        for i in 0..nums.len() - 1 {
            jump_max = jump_max.max(i + nums[i] as usize);
            if end >= nums.len() - 1 {
                break;
            }
            if i == end {
                end = jump_max;
                count += 1;
            }
        }
        count
    }
}   

fn main() {
    println!("Hello, world!");
}
