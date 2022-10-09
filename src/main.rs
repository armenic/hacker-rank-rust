fn main() {
    let strings = vec!["AAAA", "BBBBB", "ABABABAB", "BABABA", "AAABBB"];

    for s in strings {
        println!("Answer is: {}", alternating_characters(s))
    }
}

fn alternatingCharacters(s: &str) -> i32 {
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
    println!("The stack is {:?}", stack);
    println!("Original length is {l_original}");
    println!("Stack length is {l_stack}");
    l_original - l_stack
}
