struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut ans = vec![-1, -1];

        fn find_left(nums: &Vec<i32>, target: i32) -> i32 {
            let mut left = 0;
            let mut right = nums.len();
            while left < right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&target) {
                    Ordering::Less => left = mid + 1,
                    Ordering::Equal => right = mid,
                    Ordering::Greater => right = mid,
                }
            }
            if left == nums.len() || nums[left] != target {
                -1
            } else {
                left as i32
            }
        }

        fn find_right(nums: &Vec<i32>, target: i32) -> i32 {
            let mut left = 0;
            let mut right = nums.len();
            while left < right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&target) {
                    Ordering::Less => left = mid + 1,
                    Ordering::Equal => left = mid + 1,
                    Ordering::Greater => right = mid,
                }
            }
            if left == 0 || nums[left - 1] != target {
                -1
            } else {
                (left - 1) as i32
            }
        }

        ans[0] = find_left(&nums, target);
        ans[1] = find_right(&nums, target);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
