fn main() {
    assert_eq!(1, beautifulBinaryString("010"));
    assert_eq!(2, beautifulBinaryString("0101010"));
    assert_eq!(0, beautifulBinaryString("01100"));
    assert_eq!(3, beautifulBinaryString("0100101010"));
    println!("Success!")
}

#[allow(non_snake_case)]
fn beautifulBinaryString(b: &str) -> i32 {
    let matches: Vec<&str> = b.matches("010").collect();
    // println!("{}", matches.len());
    matches.len() as i32
}
