struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut used = vec![false; nums.len()];
        let mut path = vec![];
        let mut ans = vec![];

        fn backtracking(
            nums: &[i32],
            start: usize,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            used: &mut Vec<bool>,
        ) {
            ans.push(path.clone());
            if start == nums.len() {
                return;
            }
            for i in start..nums.len() {
                if used[i] {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                path.push(nums[i]);
                used[i] = true;
                backtracking(nums, i + 1, path, ans, used);
                path.pop();
                used[i] = false;
            }
        }
        let mut nums = nums;
        nums.sort();
        backtracking(&nums, 0, &mut path, &mut ans, &mut used);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
