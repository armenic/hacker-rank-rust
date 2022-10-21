use std::collections::HashMap;


/* cSpell:disable */
fn main() {
    assert_eq!(4, lonelyinteger(&[1, 2, 3, 4, 3, 2, 1]));
    assert_eq!(1, lonelyinteger(&[1]));
    assert_eq!(2, lonelyinteger(&[1, 1, 2]));
    assert_eq!(2, lonelyinteger(&[0, 0, 1, 2, 1]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn lonelyinteger(a: &[i32]) -> i32 {
    let mut dir = HashMap::new();

    for x in a {
        let count = dir.entry(*x).or_insert(0);
        *count += 1
    }

    // println!("{:#?}", dir);

    let mut answer = 0;
    for (k, v) in dir.iter() {
        if v == &1 {
            answer = *k;
        }
    }
    return answer;
}
