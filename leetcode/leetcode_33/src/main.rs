struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            let left_value = nums[left];
            let mid_value = nums[mid];
            let right_value = nums[right - 1];

            if mid_value == target {
                return mid as i32;
            }
            if left_value == target {
                return left as i32;
            }
            if right_value == target {
                return (right - 1) as i32;
            }

            if left_value < mid_value {
                if target > left_value && target < mid_value {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if target > mid_value && target < right_value {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
