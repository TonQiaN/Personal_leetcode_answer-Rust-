struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars = word.chars().collect();
        let n = board.len();
        let m = board.first().map_or(0, |r0| r0.len());
        let mut visited = vec![vec![false; m]; n];

        fn dfs(
            i: usize,
            j: usize,
            board: &Vec<Vec<char>>,
            start: usize,
            word: &Vec<char>,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if board[i][j] == word[start] {
                let start = start + 1;
                visited[i][j] = true;
                if start == word.len() {
                    return true;
                }
                let neighbours = [
                    (i + 1, j),
                    (i, j + 1),
                    (i.wrapping_sub(1), j),
                    (i, j.wrapping_sub(1)),
                ];
                let mut result = false;
                for (i1, j1) in neighbours {
                    if i1 < board.len() && j1 < board[0].len() && !visited[i1][j1] {
                        result |= dfs(i1, j1, board, start, word, visited);
                    }
                }
                visited[i][j] = false;
                return result;
            }
            false
        }

        for i in 0..n {
            for j in 0..m {
                if dfs(i, j, &board, 0, &word_chars, &mut visited) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
