struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {

        fn backtracking(n: i32, k: usize, start: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == k {
                ans.push(path.clone());
                return;
            }
            let need = (k - path.len()) as i32;
            let max_i = n - need + 1;
            for i in start..=max_i {
                path.push(i);
                backtracking(n, k, i + 1, path, ans);
                path.pop();
            }
        }

        let mut ans = Vec::new();
        let mut path = Vec::with_capacity(k as usize);
        backtracking(n, k as usize, 1, &mut path, &mut ans);
        ans
    }
}


fn main() {
    println!("Hello, world!");
}
