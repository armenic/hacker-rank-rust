fn main() {
    assert_eq!(4, theLoveLetterMystery("abcd"));
    assert_eq!(2, theLoveLetterMystery("abc"));
    assert_eq!(2, theLoveLetterMystery("cde"));
    assert_eq!(0, theLoveLetterMystery("abcba"));
    assert_eq!(2, theLoveLetterMystery("cba"));

    println!("Success!")
}

#[allow(non_snake_case)]
fn theLoveLetterMystery(s: &str) -> i32 {
    let mut ans: i32 = 0;
    let bts = s.bytes();
    let mut stack: Vec<i32> = Vec::new();
    for b in bts {
        stack.push(b as i32)
    }
    // println!("{:?}", stack);

    let l = stack.len();
    // let is_even = l % 2 == 0;
    let n_items = (l as f32 / 2.0).floor() as i32;

    for i in 0..n_items {
        let lhs = stack[i as usize];
        let rhs = stack[stack.len() - i as usize - 1];
        let diff = (lhs as i32 - rhs as i32).abs();
        ans = ans + diff;
        // println!("{:?}", lhs);
        // println!("{:?}", rhs);
        // println!("{:?}", ans);
    }

    ans
}
