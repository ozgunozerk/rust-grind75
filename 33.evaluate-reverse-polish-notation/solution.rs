impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            if token == "+" || token == "-" || token == "*" || token == "/" {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => panic!(),
                })
            } else {
                stack.push(token.parse::<i32>().unwrap())
            }
        }

        stack.pop().unwrap()
    }
}
