use core::num;

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        ans.push(vec![1]);
        let num_rows = num_rows as usize;
        for i in 1..num_rows {
            let row = std::iter::once(1)
            .chain(ans[i - 1].windows(2).map(|s| s[0] + s[1]))
            .chain(std::iter::once(1))
            .collect();
            ans.push(row);
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
