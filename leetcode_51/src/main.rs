use std::path;

struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        let n = n as usize;
        let mut ans = vec![];
        let mut chessboard = vec![vec!['.'; n]; n];
        let mut cols = HashSet::new();
        let mut pos_diag = HashSet::new();
        let mut neg_diag = HashSet::new();

        fn backtracking(
            n: &usize,
            chessboard: &mut Vec<Vec<char>>,
            ans: &mut Vec<Vec<String>>,
            cols: &mut HashSet<i32>,
            pos_diag: &mut HashSet<i32>,
            neg_diag: &mut HashSet<i32>,
            cur_row: i32,
        ) {
            if cur_row as usize == *n {
                ans.push(chessboard.iter().map(|row| row.iter().collect()).collect());
                return;
            }

            for i in 0..*n as i32 {
                if cols.contains(&i)
                    || pos_diag.contains(&(cur_row - i))
                    || neg_diag.contains(&(cur_row + i))
                {
                    continue;
                }
                chessboard[cur_row as usize][i as usize] = 'Q';
                cols.insert(i);
                pos_diag.insert(cur_row - i);
                neg_diag.insert(cur_row + i);
                backtracking(n, chessboard, ans, cols, pos_diag, neg_diag, cur_row + 1);
                chessboard[cur_row as usize][i as usize] = '.';
                cols.remove(&i);
                pos_diag.remove(&(cur_row - i));
                neg_diag.remove(&(cur_row + i));
            }
        }

        backtracking(
            &n,
            &mut chessboard,
            &mut ans,
            &mut cols,
            &mut pos_diag,
            &mut neg_diag,
            0,
        );
        ans
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        let mut path = vec![vec!['.'; n as usize]; n as usize];
        let mut ans = vec![];
        let mut cols = HashSet::new();
        let mut pos_diag = HashSet::new();
        let mut neg_diag = HashSet::new();

        fn backtracking(
            n: i32,
            path: &mut Vec<Vec<char>>,
            ans: &mut Vec<Vec<String>>,
            cols: &mut HashSet<i32>,
            pos_diag: &mut HashSet<i32>,
            neg_diag: &mut HashSet<i32>,
            cur_row: i32,
        ) {
            if cur_row == n {
                ans.push(
                    path.clone()
                        .into_iter()
                        .map(|row| row.iter().collect())
                        .collect(),
                );
                return;
            }
            for i in 0..n {
                if cols.contains(&i)
                    || pos_diag.contains(&(cur_row + i))
                    || neg_diag.contains(&(cur_row - i))
                {
                    continue;
                }

                path[cur_row as usize][i as usize] = 'Q';
                cols.insert(i);
                pos_diag.insert(cur_row + i);
                neg_diag.insert(cur_row - i);
                backtracking(n, path, ans, cols, pos_diag, neg_diag, cur_row + 1);
                path[cur_row as usize][i as usize] = '.';
                cols.remove(&i);
                pos_diag.remove(&(cur_row + i));
                neg_diag.remove(&(cur_row - i));
            }
        }

        backtracking(n, &mut path, &mut ans, &mut cols, &mut pos_diag, &mut neg_diag, 0);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
