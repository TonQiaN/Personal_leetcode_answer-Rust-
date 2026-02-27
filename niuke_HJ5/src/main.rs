use std::io::{self, Read};

fn main() {
    let (mut input, mut out) = (String::new(), String::new());
    io::stdin().read_to_string(&mut input).unwrap();
    let it = input.split_whitespace().next().unwrap();
    let mut ans = 0;
    for ch in it.chars().skip(2) {
        let val = match ch {
            '0'..'9' => ch as i64 - b'0' as i64,
            'A'..'F' => ch as i64 - b'A' as i64 + 10,
            _ => 0,
        };
        ans = ans * 16 + val;
    }
    print!("{}", ans)
}




