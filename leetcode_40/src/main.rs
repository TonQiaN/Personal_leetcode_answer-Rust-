struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            candidates: &[i32],
            target: &i32,
            cur_sum: &mut i32,
            start: &mut usize,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            used: &mut Vec<bool>,
        ) {
            if *cur_sum == *target {
                ans.push(path.clone());
                return;
            }
            for (i, &num) in candidates.iter().enumerate().skip(*start) {
                if *cur_sum + num > *target {
                    break;
                }
                if i > 0 && candidates[i] == candidates[i - 1] && used[i - 1] == false {
                    continue;
                }
                if used[i] {
                    continue;
                }
                path.push(num);
                used[i] = true;
                *cur_sum += num;
                let old = *start;
                *start = i + 1;
                backtracking(candidates, target, cur_sum, start, path, ans, used);
                *start = old;
                *cur_sum -= num;
                path.pop();
                used[i] = false;
            }
        }
        let mut candidates = candidates;
        let mut cur_sum = 0;
        let mut start = 0;
        let mut path = vec![];
        let mut ans = vec![];
        let mut used = vec![false; candidates.len()];
        candidates.sort();
        backtracking(&candidates, &target, &mut cur_sum, &mut start, &mut path, &mut ans, &mut used);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
