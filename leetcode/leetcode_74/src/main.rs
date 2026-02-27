struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;
        let n = matrix.len();
        let m = matrix.first().map_or(0, |r0| r0.len());
        let (mut left, mut right) = (0, n);
        while left < right {
            let mid = left + (right - left) / 2;
            match matrix[mid][0].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return true,
                Ordering::Greater => right = mid,
            }
        }
        if left == 0 {
            return false;
        }
        let i = left - 1;
        let (mut left, mut right) = (0, m);
        while left < right {
            let mid = left + (right - left) / 2;
            match matrix[i][mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return true,
                Ordering::Greater => right = mid,
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
