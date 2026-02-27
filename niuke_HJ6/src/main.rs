use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut num = input
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut cnt = 2;
    let mut out = vec![];
    loop {
        if cnt * cnt > num {
            break;
        }
        if num % cnt == 0 {
            out.push(cnt);
            num /= cnt;
        } else {
            cnt += 1;
        }
    }
    if num > 1 {
        out.push(cnt);
    }
    let out = out.iter().map(|num| num.to_string()).collect::<Vec<String>>();
    print!("{}", out.join(" "));
}
