struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((10001, 0), |acc, &price| {
                let (min, current_maximum_profit) = acc;
                (min.min(price), current_maximum_profit.max(price - min))
            })
            .1
    }
}
