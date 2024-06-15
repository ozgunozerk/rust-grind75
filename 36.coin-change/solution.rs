use std::cmp::min;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut min_table = vec![2147483646; amount as usize + 1]; // MAX - 1
        min_table[0] = 0;

        for coin in coins.into_iter() {
            for i in (coin as usize)..=amount {
                min_table[i] = min(min_table[i], min_table[i - coin as usize] + 1);
            }
        }

        return match min_table[amount] {
            2147483646 => -1, // MAX - 1
            x @ _ => x,
        };
    }
}
