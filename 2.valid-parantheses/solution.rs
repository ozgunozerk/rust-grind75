impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        let check = s.chars().into_iter().all(|c| match c {
            '(' | '[' | '{' => {
                stack.push(c);
                true
            }
            ')' => stack.pop() == Some('('),
            ']' => stack.pop() == Some('['),
            '}' => stack.pop() == Some('{'),
            _ => true,
        });

        check && stack.is_empty()
    }
}
