struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((1, 0), |(curr, prev), _| (curr + prev, curr)).0
    }


    // pub fn climb_stairs(n: i32) -> i32 {
    //     if n < 2 {
    //         return 1;
    //     }
    //     let mut prev = 1;
    //     let mut prev_prev = 1;
    //     let mut curr = 0;
    //     for _ in 2..n + 1 {
    //         curr = prev + prev_prev;
    //         prev_prev = prev;
    //         prev = curr;
    //     }
    //     curr
    // }


    // pub fn climb_stairs(n: i32) -> i32 {
    //     let n  = n as usize;
    //     let mut steps = vec![-1; n + 1];
    //     steps[0] = 1;
    //     steps[1] = 1;
    //     fn climb(n: usize, steps: &mut Vec<i32>) -> i32 {
    //         if steps[n] != -1 {
    //             return steps[n];
    //         } else {
    //             steps[n] = climb(n - 1, steps) + climb(n - 2, steps);
    //             steps[n]
    //         }
    //     }
    //     climb(n, &mut steps)
    // }
}

fn main() {
    println!("Hello, world!");
}
