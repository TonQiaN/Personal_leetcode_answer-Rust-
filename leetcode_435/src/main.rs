struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut count = 0;
        intervals.sort_by_key(|x| x[1]);
        let mut min_right = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] < min_right {
                count += 1;
            } else {
                min_right = intervals[i][1];
            }
        }

        count
    }
}
fn main() {
    println!("Hello, world!");
}
