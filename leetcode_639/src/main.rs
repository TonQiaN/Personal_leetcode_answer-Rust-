struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        const MOD: i64 = 1_000_000_007;
        s_bytes
            .iter()
            .rfold((1i64, 0i64, None::<u8>), |(dp1, dp2, next), &c| {
                // single
                let mut curr = match c {
                    b'0' => 0,
                    b'*' => 9 * dp1,
                    _ => dp1,
                };

                // two
                if let Some(next) = next {
                    let add = match (c, next) {
                        (b'*', b'*') => 15 * dp2,
                        (b'*', next) => {
                            if (next - b'0') <= 6 {
                                2 * dp2
                            } else {
                                1 * dp2
                            }
                        },
                        (c, b'*') => match c {
                            b'1' => 9 * dp2,
                            b'2' => 6 * dp2,
                            _ => 0,
                        },
                        (c, next) => {
                            let two_num = (c - b'0') * 10 + next - b'0';
                            if two_num <= 26 && two_num >= 10 {
                                dp2
                            } else {
                                0
                            }
                        },
                    };
                    curr += add;
                }
                (curr % MOD, dp1 % MOD, Some(c))
            })
            .0 as i32
    }
}

fn main() {
    println!("Hello, world!");
}
