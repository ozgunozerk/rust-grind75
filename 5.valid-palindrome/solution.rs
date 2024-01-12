impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        let mut reverse = s.chars().rev().collect::<String>();

        s == reverse
    }
}
