struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ans = vec![];
        fn backtracking(nums: &[i32], start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() > 1 {
                ans.push(path.clone());
            }
            if start == nums.len() {
                return;
            }

            let mut used = HashSet::new();
            for i in start..nums.len() {
                if used.contains(&nums[i]) {
                    continue;
                }
                match path.last() {
                    None => {
                        path.push(nums[i]);
                        used.insert(nums[i]);
                    }
                    Some(&last_num) => {
                        if nums[i] >= last_num {
                            path.push(nums[i]);
                            used.insert(nums[i]);
                        } else {
                            continue;
                        }
                    }
                }
                backtracking(nums, i + 1, path, ans);
                path.pop();
            }
        }
        backtracking(&nums, 0, &mut path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
