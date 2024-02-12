impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let odd_mask = s.as_bytes().iter().fold(0, |x, y| y ^ (1u64 << (x ^ 63)));

        s.len() as i32 - odd_mask.count_ones() as i32 + if odd_mask == 0 { 0 } else { 1 }
    }
}
