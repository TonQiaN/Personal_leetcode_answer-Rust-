struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut wallet = vec![0; 3];
        for bill in bills {
            match bill {
                5 => wallet[0] += 1,
                10 => {
                    if wallet[0] > 0 {
                        wallet[0] -= 1;
                        wallet[1] += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if wallet[0] > 0 && wallet[1] > 0 {
                        wallet[0] -= 1;
                        wallet[1] -= 1;
                        wallet[2] += 1;
                    } else if wallet[0] >= 3 {
                        wallet[0] -= 3;
                        wallet[2] += 1;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
