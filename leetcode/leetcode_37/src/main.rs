struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, k: char) -> bool {
            // row
            for j in 0..board.len() {
                if board[row][j] == k {
                    return false;
                }
            }

            for i in 0..board.len() {
                if board[i][col] == k {
                    return false;
                }
            }

            let row = row - row % 3;
            let col = col - col % 3;
            for i in row..row + 3 {
                for j in col..col + 3 {
                    if board[i][j] == k {
                        return false;
                    }
                }
            }
            true
        }

        fn backtracking(board: &mut Vec<Vec<char>>) -> bool {
            let n = board.len();
            for row in 0..n {
                for col in 0..n {
                    if board[row][col] == '.' {
                        for k in 1..=9 {
                            // let k = (b'0' + k as u8) as char;
                            let k = (k as u8) as char;
                            if is_valid(&board, row, col, k) {
                                board[row][col] = k;
                                let result = backtracking(board);
                                if result {
                                    return true;
                                } else {
                                    board[row][col] = '.';
                                }
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }

        backtracking(board);
    }
}

fn main() {
    println!("Hello, world!");
}
