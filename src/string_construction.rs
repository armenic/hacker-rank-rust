fn main() {
    assert_eq!(4, string_construction("abcd"));
    assert_eq!(2, string_construction("abab"));
    assert_eq!(3, string_construction("abcabc"));
    assert_eq!(4, string_construction("scfg"));
    assert_eq!(2, string_construction("bccb"));
    println!("Success!")
}

fn string_construction(s: &str) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let s_copy: Vec<char> = s.clone().chars().collect();
    let mut new_s: &str = s.clone();

    let mut cost = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let c = s_copy[i];
        // As long as the current character is in the stack we can just append
        // with no cost
        if !stack.contains(&c) {
            cost = cost + 1;
        }
        stack.push(c);
        // Remove the first character
        if new_s.len() > 0 {
            new_s = &new_s[1..]
        }

        i = i + 1;
    }

    cost
}
