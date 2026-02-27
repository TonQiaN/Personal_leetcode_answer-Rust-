struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        const NEITHER: u8 = 0b000;
        const PACIFIC: u8 = 0b001;
        const ATLANTIC: u8 = 0b010;
        const BOTH: u8 = 0b011;
        const ADDED: u8 = 0b111;

        let n = heights.len();
        let m = heights.first().map_or(0, |r1| r1.len());
        let mut reached = vec![vec![NEITHER; m]; n];
        let mut queue = VecDeque::new();
        let mut ans = vec![];

        for r in 0..n {
            reached[r][0] |= PACIFIC;
            queue.push_back((r, 0, PACIFIC));

            reached[r][m - 1] |= ATLANTIC;
            queue.push_back((r, m - 1, ATLANTIC));
        }

        for c in 0..m {
            reached[0][c] |= PACIFIC;
            queue.push_back((0, c, PACIFIC));

            reached[n - 1][c] |= ATLANTIC;
            queue.push_back((n - 1, c, ATLANTIC));
        }

        while let Some((r1, c1, ocean)) = queue.pop_front() {
            let h = heights[r1][c1];
            let neighbours = [
                (r1 + 1, c1),
                (r1, c1 + 1),
                (r1.wrapping_sub(1), c1),
                (r1, c1.wrapping_sub(1)),
            ];
            for (r2, c2) in neighbours {
                if r2 < n && c2 < m && heights[r2][c2] >= h && reached[r2][c2] & ocean == 0 {
                    reached[r2][c2] |= ocean;
                    queue.push_back((r2, c2, ocean));
                }
            }
        }

        for r in 0..n {
            for c in 0..m {
                if reached[r][c] == BOTH {
                    reached[r][c] = ADDED;
                    ans.push(vec![r as i32, c as i32]);
                }
            }
        }
        ans
    }
}

// impl Solution {
//     pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         use std::collections::VecDeque;
//         let mut ans = vec![];
//         let (n, m) = (heights.len(), heights[0].len());
//         let mut grid = vec![vec![0; m]; n];
//         let mut queue = VecDeque::new();
//         // left and top
//         for i in 0..m {
//             grid[0][i] = 1;
//             queue.push_back((0, i));
//         }
//         for i in 1..n {
//             grid[i][0] = 1;
//             queue.push_back((i, 0));
//         }
//         while let Some((r, c)) = queue.pop_front() {
//             let h = heights[r][c];
//             let mut try_push = |nr: usize, nc: usize| {
//                 if heights[nr][nc] >= h && grid[nr][nc] == 0 {
//                     grid[nr][nc] = 1;
//                     queue.push_back((nr, nc));
//                 }
//             };
//             if r + 1 < n {
//                 try_push(r + 1, c);
//             }
//             if r > 0 {
//                 try_push(r - 1, c);
//             }
//             if c + 1 < m {
//                 try_push(r, c + 1);
//             }
//             if c > 0 {
//                 try_push(r, c - 1);
//             }
//         }

//         // right and bottom
//         for i in 0..m {
//             grid[n - 1][i] = if grid[n - 1][i] == 1 { 3 } else { 2 };
//             queue.push_back((n - 1, i));
//         }
//         for i in 0..n - 1 {
//             grid[i][m - 1] = if grid[i][m - 1] == 1 { 3 } else { 2 };
//             queue.push_back((i, m - 1));
//         }

//         while let Some((r, c)) = queue.pop_front() {
//             let h = heights[r][c];

//             // helper: try visit neighbor for atlantic
//             let mut try_push = |nr: usize, nc: usize| {
//                 if heights[nr][nc] >= h {
//                     match grid[nr][nc] {
//                         0 => {
//                             grid[nr][nc] = 2;
//                             queue.push_back((nr, nc));
//                         } // first atl
//                         1 => {
//                             grid[nr][nc] = 3;
//                             queue.push_back((nr, nc));
//                         } // become both
//                         _ => {} // 2 or 3 already atl-visited, don't push again
//                     }
//                 }
//             };

//             if r + 1 < n {
//                 try_push(r + 1, c);
//             }
//             if r > 0 {
//                 try_push(r - 1, c);
//             }
//             if c + 1 < m {
//                 try_push(r, c + 1);
//             }
//             if c > 0 {
//                 try_push(r, c - 1);
//             }
//         }

//         for i in 0..n {
//             for j in 0..m {
//                 if grid[i][j] == 3 {
//                     ans.push(vec![i as i32, j as i32]);
//                 }
//             }
//         }

//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
