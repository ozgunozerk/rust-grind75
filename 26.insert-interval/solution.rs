use std::cmp::{max, min};

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let res = vec![new_interval];

        intervals.iter().fold(res, |mut acc, interval| {
            let last: &mut Vec<i32> = acc.last_mut().unwrap();
            if interval[1] < last[0] {
                acc.insert(acc.len() - 1, interval.clone());
            } else if interval[0] > last[1] {
                acc.push(interval.clone());
            } else {
                last[0] = min(last[0], interval[0]);
                last[1] = max(last[1], interval[1]);
            }
            acc
        })
    }
}
