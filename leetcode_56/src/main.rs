use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return intervals;
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut intervals = intervals;
        intervals.sort_by_key(|x| x[0]);
        // intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        for interval in intervals {
            if ans.is_empty() || ans.last().unwrap()[1] < interval[0] {
                ans.push(interval);
            } else {
                ans.last_mut().unwrap()[1] = i32::max(ans.last().unwrap()[1], interval[1]);
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
