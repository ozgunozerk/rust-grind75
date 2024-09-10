impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = height.iter().copied().enumerate().peekable();
        let mut right = height.iter().copied().enumerate().rev().peekable();

        loop {
            let Some((left_index, left_height)) = left.peek().copied() else { break };
            let Some((right_index, right_height)) = right.peek().copied() else { break };

            let area = (right_index - left_index) as i32 * right_height.min(left_height);
            res = area.max(res);

            if left_index == right_index {
                break
            } else if left_height <= right_height {
                left.next();
            } else if left_height > right_height {
                right.next();
            }
        }

        res
    }
}
