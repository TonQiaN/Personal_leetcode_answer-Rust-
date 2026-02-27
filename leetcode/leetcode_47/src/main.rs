struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ans = vec![];
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
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }

                used[i] = true;
                path.push(*num);
                backtracking(nums, used, path, ans);
                path.pop();
                used[i] = false;
            }
        }

        let mut nums = nums;
        nums.sort();
        backtracking(&nums, &mut used, &mut path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
