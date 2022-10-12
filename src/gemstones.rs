use std::collections::HashMap;

fn main() {
    assert_eq!(
        2,
        gemstones(&["abc".to_string(), "abc".to_string(), "bc".to_string()])
    );
    assert_eq!(
        2,
        gemstones(&[
            "abcdde".to_string(),
            "baccd".to_string(),
            "eeabg".to_string()
        ])
    );
    println!("Success!")
}

#[allow(non_snake_case)]
fn gemstones(arr: &[String]) -> i32 {
    let mut map = HashMap::new();

    for s in arr {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if !stack.contains(&c) {
                stack.push(c)
            }
        }

        for c in stack {
            let count = map.entry(c).or_insert(0);
            *count += 1
        }
    }

    let mut count = 0;
    for v in map.values() {
        if v == &arr.len() {
            count = count + 1
        }
    }

    count
}
