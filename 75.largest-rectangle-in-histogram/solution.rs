impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;

        // insert 0 to both start and end for the heights
        // no need to process remaining items in the stack
        heights.insert(0, 0);
        heights.push(0);

        let mut stack = vec![0];
        let mut max_area = 0;

        for (idx, &height) in heights.iter().enumerate() {
            let mut block_start_idx = *stack.last().unwrap();

            while height < heights[block_start_idx] {
                let block_height = heights[stack.pop().unwrap()];
                block_start_idx = *stack.last().unwrap();

                let block_width = (idx - block_start_idx - 1) as i32;

                max_area = max_area.max(block_width * block_height);
            }

            stack.push(idx);
        }

        max_area
    }
}
