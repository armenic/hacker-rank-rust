/* cSpell:disable */
fn main() {
    assert_eq!("Second", gameOfStones(1));
    assert_eq!("First", gameOfStones(2));
    assert_eq!("First", gameOfStones(3));
    assert_eq!("First", gameOfStones(4));
    assert_eq!("First", gameOfStones(5));
    assert_eq!("First", gameOfStones(6));
    assert_eq!("Second", gameOfStones(7));
    assert_eq!("First", gameOfStones(10));

    println!("Success!")
}

#[allow(non_snake_case)]
fn gameOfStones(n: i32) -> String {
    if n % 7 < 2 {
        return "Second".to_string();
    } else {
        return "First".to_string();
    }
}
