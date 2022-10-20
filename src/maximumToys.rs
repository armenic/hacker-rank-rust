/* cSpell:disable */
fn main() {
    assert_eq!(3, maximumToys(&[4, 2, 3, 1], 7));
    assert_eq!(4, maximumToys(&[1, 12, 5, 111, 200, 1000, 10], 50));

    println!("Success!")
}

#[allow(non_snake_case)]
fn maximumToys(prices: &[i32], k: i32) -> i32 {
    let mut prices_sorted = Vec::from(prices);
    prices_sorted.sort();

    let mut spent = 0;
    let mut result: i32 = 0;

    for (i, v) in prices_sorted.iter().enumerate() {
        spent = spent + v;
        if spent > k {
            result = i as i32;
            break;
        }
    }

    // println!("{:?} {spent} {result}", prices_sorted);
    result
}
