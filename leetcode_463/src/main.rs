struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let n = grid.len();
        let m = grid.first().map_or(0, |r0| r0.len());

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 {
                    let neighbours = [
                        (r + 1, c),
                        (r, c + 1),
                        (r.wrapping_sub(1), c),
                        (r, c.wrapping_sub(1)),
                    ];
                    for (r1, c1) in neighbours {
                        if r1 < n && c1 < m {
                            if grid[r1][c1] == 0 {
                                ans += 1;
                            }
                        } else {
                            ans += 1;
                        }
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
