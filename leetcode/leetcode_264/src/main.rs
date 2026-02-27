struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[1] = 1;
        let (mut i2, mut i3, mut i5) = (1usize, 1usize, 1usize);
        for i in 2..(n + 1) as usize {
            let a = dp[i2] * 2;
            let b = dp[i3] * 3;
            let c = dp[i5] * 5;
            let curr = a.min(b).min(c);
            dp[i] = curr;
            if a == curr {
                i2 += 1;
            }
            if b == curr {
                i3 += 1;
            }
            if c == curr {
                i5 += 1;
            }
        }
        dp[n as usize]
    }
}

fn main() {
    println!("Hello, world!");
}
