impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums[0];

        for value in nums {
            if count == 0 {
                candidate = value;
            }

            if value == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate
    }
}
