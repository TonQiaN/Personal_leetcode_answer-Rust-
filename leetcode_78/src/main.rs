use std::path;

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ans = vec![];
        fn backtracking(nums: &[i32], start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            ans.push(path.clone());
            if nums.is_empty() {
                return;
            }
            for i in start..nums.len() {
                path.push(nums[i]);
                backtracking(nums, i + 1, path, ans);
                path.pop();
            }
        }
        backtracking(&nums, 0, &mut path, &mut ans);
        ans
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];
        let mut path = vec![];

        fn backtracking(nums: &Vec<i32>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, start: usize) {
            for i in start..nums.len() {
                path.push(nums[i]);
                ans.push(path.clone());
                backtracking(nums, path, ans, i + 1);
                path.pop();
            }
        }
        backtracking(&nums, &mut path, &mut ans, 0);

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
