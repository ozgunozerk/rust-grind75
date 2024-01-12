impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut best, mut min) = (0, prices[0]);

        for i in 1..prices.len() {
            min = prices[i].min(min);
            best = best.max(prices[i] - min);
        }

        best
    }
}
