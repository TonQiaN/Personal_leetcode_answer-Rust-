
struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut ans = vec![];
        let mut nums = nums;
        nums.sort();
        if nums.len() < 4 {
            return ans;
        }
        for k in 0..nums.len() - 3 {
            if nums[k] > target && target > 0 {
                break;
            }
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }
            for i in k + 1..nums.len() - 2 {
                if nums[i] + nums[k] > target && target > 0 {
                    break;
                }
                if i > k + 1 && nums[i] == nums[i - 1] {
                    continue;
                }
                let (mut left, mut right) = (i + 1, nums.len() - 1);
                while left < right {
                    match (nums[k] as i64 +  nums[i] as i64 +  nums[left] as i64 + nums[right] as i64).cmp(&(target as i64)) {
                        Ordering::Less => {
                            left += 1;
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                        },
                        Ordering::Equal => {
                            ans.push(vec![nums[k], nums[i], nums[left], nums[right]]);
                            left += 1;
                            right -= 1;
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        },
                        Ordering::Greater => {
                            right -= 1;
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        },
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
