struct Solution {}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut grid = grid;
        let mut ans = vec![0; hits.len()];
        let mut queue = VecDeque::new();
        let (n, m) = (grid.len(), grid.first().map_or(0, |r0| r0.len()));
        for hit in &hits {
            let (r, c) = (hit[0], hit[1]);
            grid[r as usize][c as usize] -= 1;
        }

        for i in 0..m {
            if grid[0][i] == 1 {
                grid[0][i] = 2;
                queue.push_back((0usize, i));
            }
        }

        while let Some((r, c)) = queue.pop_front() {
            let neighbours = [
                (r + 1, c),
                (r, c + 1),
                (r.wrapping_sub(1), c),
                (r, c.wrapping_sub(1)),
            ];
            for (r1, c1) in neighbours {
                if r1 < n && c1 < m && grid[r1][c1] == 1 {
                    grid[r1][c1] = 2;
                    queue.push_back((r1, c1));
                }
            }
        }

        for i in (0..hits.len()).rev() {
            let (r, c) = (hits[i][0] as usize, hits[i][1] as usize);
            grid[r][c] += 1;
            if grid[r][c] == 1 {
                let mut cnt = 0;
                let mut flag = false;
                let neighbours = [
                    (r + 1, c),
                    (r, c + 1),
                    (r.wrapping_sub(1), c),
                    (r, c.wrapping_sub(1)),
                ];
                for (r1, c1) in neighbours {
                    if r1 < n && c1 < m && grid[r1][c1] == 2 {
                        flag = true;
                    }
                }
                if r == 0 {
                    flag = true;
                }

                if flag {
                    grid[r][c] = 2;
                    queue.push_back((r, c));
                    while let Some((r2, c2)) = queue.pop_front() {
                        let neighbours = [
                            (r2 + 1, c2),
                            (r2, c2 + 1),
                            (r2.wrapping_sub(1), c2),
                            (r2, c2.wrapping_sub(1)),
                        ];
                        for (r3, c3) in neighbours {
                            if r3 < n && c3 < m && grid[r3][c3] == 1 {
                                grid[r3][c3] = 2;
                                cnt += 1;
                                queue.push_back((r3, c3));
                            }
                        }
                    }
                }

                ans[i] = cnt;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
