impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut res = Vec::new();
        let mut idx = 0;
        for interval in intervals {
            if res.is_empty() {
                res.push(interval);
                continue;
            }

            if res[idx][1] >= interval[0] {
                if res[idx][1] < interval[1] {
                    res[idx][1] = interval[1];
                }
            } else {
                res.push(interval);
                idx += 1;
            }
        }

        res
    }
}
