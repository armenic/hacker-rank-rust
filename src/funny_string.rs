fn main() {
    assert_eq!("Funny", funny_string("abc"));
    assert_eq!("Funny", funny_string("lmnop"));
    assert_eq!("Funny", funny_string("acxz"));
    assert_eq!("Not Funny", funny_string("bcxz"));
    println!("Success!")
}

fn funny_string(s: &str) -> String {
    let mut stack: Vec<i8> = Vec::new();

    let bytes = s.as_bytes();

    for (i, b) in bytes.iter().enumerate() {
        if i < bytes.len() - 1 {
            let diff: i8 = (*b as i8 - bytes[i + 1] as i8).abs();
            stack.push(diff);
            // println!("{i} {b} {diff}")
        }
    }

    // println!("{:?}", stack);

    let mut i = 0;
    while i < stack.len() {
        let original_byte = stack[i];
        let reverse_byte = stack[stack.len() - i - 1];
        if original_byte != reverse_byte {
            return "Not Funny".to_string();
        }
        i = i + 1
    }

    "Funny".to_string()
}
