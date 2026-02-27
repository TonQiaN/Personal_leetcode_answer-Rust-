struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut longest = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
                longest = longest.max(count);
            } else {
                count = 1;
            }
        }
        longest
    }
}

fn main() {
    println!("Hello, world!");
}
