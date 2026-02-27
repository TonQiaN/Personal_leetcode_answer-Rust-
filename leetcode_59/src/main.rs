struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n: usize = n as usize;
        let mut ans = vec![vec![0; n]; n];
        let mut num = if n != 0 {1} else {0};
        for layer in 0..(n / 2) {
            let end = n - 1 - layer;
            for j in layer..end {
                ans[layer][j] = num;
                num += 1;
            }
            for i in layer..end {
                ans[i][end] = num;
                num += 1;
            }
            for j in (layer + 1..=end).rev() {
                ans[end][j] = num;
                num += 1;
            }
            for i in (layer + 1..=end).rev() {
                ans[i][layer] = num;
                num += 1;
            }
        }
        if n % 2 == 1 {
            let c = n / 2;
            ans[c][c] = num;
        }
        ans
    }
}

fn main() {
    let test = Solution::generate_matrix(3);
    println!("{:?}", test);
}
