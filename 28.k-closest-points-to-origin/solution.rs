impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|a| a[0].pow(2) + a[1].pow(2));
        points.into_iter().take(k as usize).collect()
    }
}
