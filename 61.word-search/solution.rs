use std::collections::HashMap;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let mut word = Vec::from_iter(word.chars());
        let mut hsmp = HashMap::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                hsmp.entry(board[i][j]).and_modify(|n| *n += 1).or_insert(1);
            }
        }

        // check if we have enough letters in the board for our letter
        let mut hsmp_clone = hsmp.clone();
        for c in &word {
            match hsmp_clone.get_mut(c) {
                None => return false,
                Some(x) => {
                    if *x == 0 {
                        return false;
                    } else {
                        *x -= 1;
                    }
                }
            }
        }

        // check if starting from beginning or the end of the word is less branching
        if hsmp.get(&word[0]).unwrap() > hsmp.get(&word[word.len() - 1]).unwrap() {
            word.reverse();
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == *word.iter().next().unwrap()
                    && Self::backtrack(&mut board, word.as_slice(), i, j)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn backtrack(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
        if word.is_empty() {
            return true;
        }
        let current_char = word[0];
        x < board.len() && y < board[0].len() && board[x][y] == current_char && {
            board[x][y] = '#';
            let rem = &word[1..];

            // iterate in 4 directions, and if any of them succeeds, return true
            let res = [0, 1, 0, !0, 0]
                .windows(2)
                .any(|w| Self::backtrack(board, rem, x.wrapping_add(w[0]), y.wrapping_add(w[1])));
            board[x][y] = current_char;
            res
        }
    }
}
