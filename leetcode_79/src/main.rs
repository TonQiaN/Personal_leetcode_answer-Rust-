struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        let n = board.len();
        let m = board.first().map_or(0, |ro| ro.len());
        let mut visited = vec![vec![false; m]; n];

        fn dfs(
            board: &Vec<Vec<char>>,
            start: usize,
            visited: &mut Vec<Vec<bool>>,
            word: &Vec<char>,
            i: usize,
            j: usize,
        ) -> bool {
            let mut result = false;
            if board[i][j] == word[start] {
                if start + 1 == word.len() {
                    return true;
                }
                visited[i][j] = true;
                let n = board.len();
                let m = board.first().map_or(0, |ro| ro.len());
                let neighbours = [
                    (i + 1, j),
                    (i, j + 1),
                    (i.wrapping_sub(1), j),
                    (i, j.wrapping_sub(1)),
                ];
                for (i_next, j_next) in neighbours {
                    if i_next < n && j_next < m && !visited[i_next][j_next] {
                        result |= dfs(board, start + 1, visited, word, i_next, j_next);
                    }
                }
            }
            visited[i][j] = false;
            result
        }

        for i in 0..n {
            for j in 0..m {
                if dfs(&board, 0, &mut visited, &word, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {}
