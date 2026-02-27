struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            k: usize,
            n: i32,
            start: i32,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            curr_sum: &mut i32
        ) {
            if *curr_sum > n {
                return;
            }
            if path.len() == k && *curr_sum == n {
                ans.push(path.clone());
                return;
            }
            let need = (k - path.len()) as i32;
            for i in start..=(9 - need + 1) {
                path.push(i);
                *curr_sum += i;
                backtracking(k, n, i + 1, path, ans, curr_sum);
                *curr_sum -= i;
                path.pop();
            }
        }
        let mut path = vec![];
        let mut ans = vec![];
        let mut curr_sum = 0;
        backtracking(k as usize, n, 1, &mut path, &mut ans, &mut curr_sum);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
