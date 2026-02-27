struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }
        let mut ans = vec![String::new(); num_rows as usize];
        let mut i = 0;
        let mut offset = -1;
        for c in s.chars() {
            if i == 0 || i == num_rows - 1 {
                offset = -offset;
            }
            ans[i as usize].push(c);
            i += offset;
        }
        ans.concat()
    }
    // pub fn convert(s: String, num_rows: i32) -> String {
    //     if num_rows == 1 || s.len() <= num_rows as usize {
    //         return s;
    //     }
    //     let s = s.chars().collect::<Vec<_>>();
    //     let mut ans = vec![vec![]; num_rows as usize];
    //     let mut i = 0;
    //     let mut offset = -1;
    //     for c in s {
    //         if i == 0 || i == num_rows - 1 {
    //             offset = -offset;
    //         }
    //         ans[i as usize].push(c);
    //         i += offset;
    //     }
    //     ans.into_iter().flatten().collect()
    // }
}

impl Solution {
    // pub fn convert(s: String, num_rows: i32) -> String {
    //     let mut ans_vec = vec![String::new(); num_rows as usize];
    //     (0..num_rows)
    //         .chain((1..num_rows - 1).rev())
    //         .cycle()
    //         .zip(s.chars())
    //         .for_each(|(i, c)| ans_vec[i as usize].push(c));
    //     ans_vec.concat()
    // }
    pub fn convert(s: String, num_rows: i32) -> String {
        (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .fold(vec![String::new(); num_rows as usize], |mut rows, (i, ch)| {
                rows[i as usize].push(ch);
                rows
            })
            .concat()
    }
}

fn main() {
    println!("Hello, world!");
}
