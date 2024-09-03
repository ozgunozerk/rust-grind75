impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = (low + high) / 2;

            if nums[mid as usize] == target {
                return mid;
            }

            // Check which side is sorted:
            // if left side is sorted
            if nums[low as usize] <= nums[mid as usize] {
                // check if the target is in the left side
                if nums[low as usize] <= target && target < nums[mid as usize] {
                    // if it is in the left side, update `high`
                    high = mid - 1;
                } else {
                    // if it is not on the left side, update `low`
                    low = mid + 1;
                }
            // if right side is sorted (left side is not sorted means, right side should be sorted)
            } else {
                // check if the target is in the right side
                if nums[mid as usize] < target && target <= nums[high as usize] {
                    // if it is in the right side, update `low`
                    low = mid + 1;
                } else {
                    // if it is not on the right side, update `high`
                    high = mid - 1;
                }
            }
        }

        -1
    }
}
