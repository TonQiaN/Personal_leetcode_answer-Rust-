use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let mut out = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.lines().next().unwrap().as_bytes();
    let n = s.len();
    let mut i = 0;

    while i < n {
        let end = (i + 8).min(n);
        for &b in &s[i..end] {
            out.push(b as char);
        }
        for _ in 0..(8-(end - i)) {
            out.push('0');
        }
        out.push('\n');
        i += 8;
    }

    print!("{}", out);
}