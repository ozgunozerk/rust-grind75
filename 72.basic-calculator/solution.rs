impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut current_number = 0;
        let mut result = 0; // Current result at this depth of parentheses
        let mut sign = 1; // 1 means positive, -1 means negative

        for c in s.chars() {
            if c.is_digit(10) {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '+' || c == '-' {
                result += sign * current_number;
                sign = if c == '+' { 1 } else { -1 };
                current_number = 0;
            } else if c == '(' {
                // Push the result and the sign onto the stack, for later
                // We push the result first, then the sign
                stack.push(result);
                stack.push(sign);

                // Reset the sign and result for the new sub-expression
                sign = 1;
                result = 0;
            } else if c == ')' {
                // End of the current sub-expression
                result += sign * current_number;
                current_number = 0;

                // The result of this sub-expression is multiplied by the sign before the '('
                result *= stack.pop().unwrap();
                // Then add the result before this sub-expression
                result += stack.pop().unwrap();
            }
        }

        result += sign * current_number;
        result
    }
}
