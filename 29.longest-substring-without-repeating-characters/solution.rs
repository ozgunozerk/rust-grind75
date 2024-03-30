use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash: HashMap<char, usize> = HashMap::new();
        let mut left = -1;

        s.chars().enumerate().fold(0, |res, (idx, c)| {
            if let Some(index) = hash.insert(c, idx) {
                left = left.max(index as i32);
            }
            res.max(idx as i32 - left)
        })
    }
}
