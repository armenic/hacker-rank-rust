fn main() {
    assert_eq!(3, alternating_characters("AAAA"));
    assert_eq!(4, alternating_characters("BBBBB"));
    assert_eq!(0, alternating_characters("ABABABAB"));
    assert_eq!(0, alternating_characters("BABABA"));
    assert_eq!(4, alternating_characters("AAABBB"));
    println!("Success!")
}

fn alternating_characters(s: &str) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let l_original = s.len() as i32;

    for i in s.chars() {
        if stack.len() > 0 {
            let last_char = stack[stack.len() - 1];
            if last_char != i {
                stack.push(i)
            }
        } else {
            stack.push(i)
        }
    }
    let l_stack = stack.len() as i32;
    l_original - l_stack
}
    