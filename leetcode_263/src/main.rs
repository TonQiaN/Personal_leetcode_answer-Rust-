struct Solution {}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        for i in [2, 3, 5] {
            while n % i == 0 {
                n /= i;
            }
        }
        n == 1
    }
}
fn main() {
    println!("Hello, world!");
}
