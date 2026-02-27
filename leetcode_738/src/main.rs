struct Solution {}

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut mul = 1;
        let mut ans = 0;
        let mut prev = 9;
        while n > 0 {
            let digit = n % 10;
            if digit <= prev {
                ans += digit * mul;
                prev = digit;
            } else {
                ans = digit * mul - 1;
                prev = digit - 1;
            }

            mul *= 10;
            n /= 10;
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
