use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set = HashSet::with_capacity(nums.len());
        !nums.iter().all(|num| hash_set.insert(num))
    }
}
