struct Solution {}

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD:i64 = 1_000_000_000 + 7;
        let mut dp = vec![0; 26];
        let mut all: i64 = 1;
        let s = s.as_bytes();
        for c in s {
            let idx = (c - b'a') as usize;
            let new = (all - dp[idx] + MOD) % MOD;
            dp[idx] = (dp[idx] + new) % MOD;
            all = (all + new) % MOD;
        }
        ((all - 1 + MOD) % MOD) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
