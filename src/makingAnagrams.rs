use std::collections::HashMap;
/* cSpell:disable */
fn main() {
    assert_eq!(6, makingAnagrams("abc", "amnop"));
    assert_eq!(7, makingAnagrams("aaabc", "aamnop"));
    assert_eq!(6, makingAnagrams("aabc", "aamnop"));
    assert_eq!(7, makingAnagrams("abc", "aamnop"));
    assert_eq!(4, makingAnagrams("cde", "abc"));
    assert_eq!(1, makingAnagrams("aabb", "abb"));

    println!("Success!")
}
/* cSpell:enable */

#[allow(non_snake_case)]
fn makingAnagrams(s1: &str, s2: &str) -> i32 {
    let mut dic: HashMap<char, i32> = HashMap::new();

    // First count in s1 then in s2. s2 counts are subtracted from s1 counts.
    // Any 0 count number means balanced frequency between string. Otherwise the
    // sum makes the answer.
    for c in s1.chars() {
        let count = dic.entry(c).or_insert(0);
        *count += 1;
    }
    for c in s2.chars() {
        let count = dic.entry(c).or_insert(0);
        *count -= 1;
    }

    let v: i32 = dic.values().map(|i| i.abs()).sum();

    println!("{:#?}", dic);
    println!("{v}");
    v
}