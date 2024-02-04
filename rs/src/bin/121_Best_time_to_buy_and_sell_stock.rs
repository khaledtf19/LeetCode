fn main() {
    let res = max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("{}", res);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut min_price, mut max_prfit) = (100000, 0);

    for &price in prices.iter() {
        max_prfit = max_prfit.max(price - min_price);
        min_price = min_price.min(price);
    }

    max_prfit
}
