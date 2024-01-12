impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let m = (low + high) / 2;
            match nums[m].cmp(&target) {
                std::cmp::Ordering::Less => low = m + 1,
                std::cmp::Ordering::Equal => return m as i32,
                std::cmp::Ordering::Greater => high = m,
            }
        }
        -1
    }
}
