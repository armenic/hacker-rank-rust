/* cSpell:disable */
fn main() {
    assert_eq!(4294967286, flippingBits(9));
    assert_eq!(2147483648, flippingBits(2147483647));
    assert_eq!(4294967294, flippingBits(1));
    assert_eq!(4294967295, flippingBits(0));
    assert_eq!(4294843839, flippingBits(123456));
    assert_eq!(4259365872, flippingBits(35601423));

    println!("Success!")
}

#[allow(non_snake_case)]
fn flippingBits(n: i64) -> i64 {
    let long_b = format!("{n:032b}");
    let new_long_b = long_b.replace("0", "8").replace("1", "0").replace("8", "1");

    let answer = i64::from_str_radix(new_long_b.as_str(), 2).unwrap();
    // println!("{new_long_b}");
    // println!("{:?}", answer);

    answer
}
