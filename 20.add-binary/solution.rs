impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a.is_empty() {
            return b;
        }
        if b.is_empty() {
            return a;
        }

        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();

        let mut next_pair = || match (a_chars.next(), b_chars.next()) {
            (None, None) => None,
            (a, b) => Some((to_value(a), to_value(b))),
        };

        let mut result = String::new();
        let mut one_in_mind = false;
        while let Some((a, b)) = next_pair() {
            let sum = if one_in_mind { a + b + 1 } else { a + b };
            one_in_mind = sum > 1;
            match sum {
                0 | 2 => result.push('0'),
                1 | 3 => result.push('1'),
                _ => panic!("Unexpected sum: {}", sum),
            }
        }
        if one_in_mind {
            result.push('1');
        }
        result.chars().rev().collect::<String>()
    }
}

fn to_value(c: Option<char>) -> u8 {
    match c {
        None => 0,
        Some('0') => 0,
        Some('1') => 1,
        Some(x) => panic! {"Unexpected char passed to value_for: {}", x},
    }
}
