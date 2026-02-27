struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy_vec = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy_vec[i] = candy_vec[i - 1] + 1;
            }
        }
        
        for j in (0..ratings.len() - 1).rev() {
            if ratings[j] > ratings[j + 1] {
                candy_vec[j] = candy_vec[j].max(candy_vec[j + 1] + 1);
            }
        }
        candy_vec.iter().sum()
    }
}

fn main() {
    println!("Hello, world!");
}
