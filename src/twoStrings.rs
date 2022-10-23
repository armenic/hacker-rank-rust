/* cSpell:disable */
fn main() {
    assert_eq!("YES", twoStrings("and", "art"));
    assert_eq!("NO", twoStrings("be", "cat"));
    assert_eq!("YES", twoStrings("hello", "world"));
    assert_eq!("NO", twoStrings("hi", "world"));

    println!("Success!")
}

#[allow(non_snake_case)]
fn twoStrings(s1: &str, s2: &str) -> String {
    for s in s1.chars() {
        if s2.contains(s) {
            return "YES".to_string();
        }
    }

    "NO".to_string()
}
