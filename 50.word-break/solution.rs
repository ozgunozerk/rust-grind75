impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let length = s.len();
        let mut dp = vec![false; length + 1];
        dp[length] = true;

        // start from the last letter
        for i in (0..length).rev() {

            // check whether a word is at the end of the string
            for word in word_dict.iter() {
                let word_len = word.len();

                // if the current (index + word_len) is longer our string, short-circuit
                if (i + word_len) <= length && &s[i.. (i + word_len)] == word.as_str() {
                    // we solved (i + word_len) part,
                    // check whether the remaining part is solvable using memoization
                    dp[i] = dp[i + word_len];
                }
                if dp[i] == true {
                    break;
                }
            }
        }

        return dp[0];
    }
}
