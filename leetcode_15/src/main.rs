struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let mut nums = nums;
        nums.sort();
        
        let mut result = vec![];
        for i in 0..nums.len() {
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            if nums[i] > 0 {
                return result;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            while left < right {
                match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                    Ordering::Less => {
                        left += 1;
                        while left < right && nums[left] == nums[left - 1]{
                            left += 1;
                        }
                    }
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[right] == nums[right + 1]{
                            right -= 1;
                        }
                        while left < right && nums[left] == nums[left - 1]{
                            left += 1;
                        }
                    }
                    Ordering::Greater => {
                        right -= 1;
                        while left < right && nums[right] == nums[right + 1]{
                            right -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
