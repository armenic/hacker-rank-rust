use std::collections::HashMap;

/* cSpell:disable */
fn main() {
    assert_eq!("YES", gameOfThrones("aabbccdd"));
    assert_eq!("YES", gameOfThrones("aaabbbb"));
    assert_eq!("NO", gameOfThrones("cdefghmnopqrstuvw"));
    assert_eq!("YES", gameOfThrones("cdcdcdcdeeeef"));
    assert_eq!("YES", gameOfThrones("a"));

    println!("Success!")
}
/* cSpell:enable */

#[allow(non_snake_case)]
fn gameOfThrones(s: &str) -> String {

    let mut odd_n = 0;
    let mut dic = HashMap::new();

    // One character string is a palindrome
    if s.len() == 1 {
        return String::from("YES");
    }

    for c in s.chars() {
        let count = dic.entry(c).or_insert();
        *count += 1;
    }

    // println!("{:#?}", dic);

    // Otherwise it must be characters with even frequency or 1 odd and the rest
    // even
    for d in dic.values() {
        let is_odd = d % 2 != 0;
        if is_odd {
            odd_n = odd_n + 1
        }
    }

    if [0, 1].contains(&odd_n) {
        return String::from("YES");
    }

    String::from("NO")
}
