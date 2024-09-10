impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut trapped_water = 0;

        left[0] = height[0];
        for i in 1..n {
            left[i] = std::cmp::max(left[i - 1], height[i]);
        }

        right[n - 1] = height[n - 1];
        for i in (0..(n - 1)).rev() {
            right[i] = std::cmp::max(right[i + 1], height[i]);
        }

        for i in 0..n {
            let min_height = std::cmp::min(left[i], right[i]);
            trapped_water += min_height - height[i];
        }

        trapped_water
    }
}
