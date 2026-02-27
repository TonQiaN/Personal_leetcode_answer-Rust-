use std::path;

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];

        fn backtracking(
            nums: &[i32],
            used: &mut Vec<bool>,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                ans.push(path.clone());
                return;
            }
            for (i, num) in nums.iter().enumerate() {
                if used[i] {
                    continue;
                }
                path.push(*num);
                used[i] = true;
                backtracking(nums, used, path, ans);
                path.pop();
                used[i] = false;
            }
        }

        backtracking(&nums, &mut used, &mut path, &mut ans);
        ans
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut used = vec![false; n];
        let mut path = vec![];
        let mut ans = vec![];

        fn backtracking(
            nums: &Vec<i32>,
            used: &mut Vec<bool>,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                ans.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if !used[i] {
                    used[i] = true;
                    path.push(nums[i]);
                    backtracking(nums, used, path, ans);
                    used[i] = false;
                    path.pop();
                }
            }
        }

        backtracking(&nums, &mut used, &mut path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
