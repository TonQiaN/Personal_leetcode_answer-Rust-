struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        let mut count = 1;
        points.sort_by_key(|x| x[1]);
        let mut end = points[0][1];        
        for i in 1..points.len() {
            if points[i][0] > end {
                count += 1;
                end = points[i][1];
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");
}
