struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (0, nums.len());
        let mut min_value = i32::MAX;
        while left < right {
            let mid = left + (right - left) / 2;
            let (left_value, mid_value, right_value) = (nums[left], nums[mid], nums[right - 1]);
            if left_value < right_value {
                min_value = min_value.min(left_value);
                break;
            }
            if left_value < mid_value {
                min_value = mid_value.min(left_value);
                left = mid + 1;
            } else {
                min_value = min_value.min(mid_value);
                right = mid;
            }
        }
        min_value
    }
}

fn main() {
    println!("Hello, world!");
}
