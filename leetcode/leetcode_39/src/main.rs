struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ans = vec![];
        fn backtracking(
            candidates: &[i32],
            target: &i32,
            path: &mut Vec<i32>,
            cur_sum: &mut i32,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if candidates.is_empty() {
                return;
            }
            if *cur_sum == *target {
                ans.push(path.clone());
                return;
            }
            for (idx, &num) in candidates.iter().enumerate() {
                if *cur_sum + num > *target {
                    break;
                }
                path.push(num);
                *cur_sum += num;
                backtracking(&candidates[idx..], target, path, cur_sum, ans);
                *cur_sum -= num;
                path.pop();
            }
        }
        let mut cur_sum = 0;
        let mut candidates = candidates;
        candidates.sort();
        backtracking(&candidates, &target, &mut path, &mut cur_sum, &mut ans);
        ans
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];
        let mut candidates = candidates;

        fn backtracking(
            candidates: &Vec<i32>,
            start: usize,
            target: i32,
            path: &mut Vec<i32>,
            cur_sum: &mut i32,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if *cur_sum == target {
                ans.push(path.clone());
            }

            for i in start..candidates.len() {
                *cur_sum += candidates[i];
                if *cur_sum > target {
                    *cur_sum -= candidates[i];
                    break;
                }
                path.push(candidates[i]);
                backtracking(candidates, i, target, path, cur_sum, ans);
                path.pop();
                *cur_sum -= candidates[i];
            }
        }

        let mut cur_sum = 0;
        candidates.sort();
        backtracking(&candidates, 0, target, &mut path, &mut cur_sum, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
