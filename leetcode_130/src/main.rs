struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        use std::collections::VecDeque;

        let (n, m) = (board.len(), board[0].len());
        let mut queue = VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                    if board[i][j] == 'O' {
                        board[i][j] = 'Y';
                        queue.push_back((i, j));
                        while !queue.is_empty() {
                            let (y, x) = queue.pop_front().unwrap();
                            if x + 1 < m && board[y][x + 1] == 'O' {
                                board[y][x + 1] = 'Y';
                                queue.push_back((y, x + 1));
                            }
                            if x > 0 && board[y][x - 1] == 'O' {
                                board[y][x - 1] = 'Y';
                                queue.push_back((y, x - 1));
                            }
                            if y + 1 < n && board[y + 1][x] == 'O' {
                                board[y + 1][x] = 'Y';
                                queue.push_back((y + 1, x));
                            }
                            if y > 0 && board[y - 1][x] == 'O' {
                                board[y - 1][x] = 'Y';
                                queue.push_back((y - 1, x));
                            }
                        }
                    }
                } else {
                    continue;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                match board[i][j] {
                    'Y' => board[i][j] = 'O',
                    'O' => board[i][j] = 'X',
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
