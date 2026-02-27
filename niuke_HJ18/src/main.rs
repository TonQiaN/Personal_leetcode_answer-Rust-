use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (mut A, mut B, mut C, mut D, mut E, mut M, mut P) = (0, 0, 0, 0, 0, 0, 0);
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let line: Vec<_> = line.split('~').collect();
        let v4 = line[0]
            .split('.')
            .filter_map(|x| x.trim().parse::<u8>().ok())
            .collect::<Vec<u8>>();
        let mask = line[1]
            .split('.')
            .filter_map(|x| x.trim().parse::<u8>().ok())
            .collect::<Vec<u8>>();

        if v4.len() != 4 || mask.len() != 4 {
            M += 1;
            continue;
        }

        if v4[0] == 0 || v4[0] == 127 {
            continue;
        }

        let mask = mask.iter().fold(0, |b, &x| b << 8 | x as u32);
        if (!mask & (!mask + 1)) != 0 || mask == 0 || mask + 1 == 0 {
            M += 1;
            continue;
        }
        match v4[0] {
            1..=127 => A += 1,
            128..=191 => B += 1,
            192..=223 => C += 1,
            224..=239 => D += 1,
            240..=255 => E += 1,
            _ => {}
        };

        match (v4[0], v4[1]) {
            (10, _) => P += 1,
            (172, 16..=31) => P += 1,
            (192, 168) => P += 1,
            _ => {}
        };
    }

    println!("{} {} {} {} {} {} {}", A, B, C, D, E, M, P);
}
