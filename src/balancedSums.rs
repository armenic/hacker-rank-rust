/* cSpell:disable */
fn main() {
    assert_eq!("YES", balancedSums(&[5, 6, 8, 11]));
    assert_eq!("YES", balancedSums(&[1]));
    assert_eq!("NO", balancedSums(&[1, 2, 3]));
    assert_eq!("YES", balancedSums(&[1, 2, 3, 3]));
    assert_eq!("YES", balancedSums(&[1, 1, 4, 1, 1]));
    assert_eq!("YES", balancedSums(&[2, 0, 0, 0]));
    assert_eq!("YES", balancedSums(&[0, 0, 2, 0]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn balancedSums(arr: &[i32]) -> String {
    let yes_answer = "YES".to_string();
    let no_answer = "NO".to_string();

    if arr.len() == 1 {
        return yes_answer;
    }

    let total: i32 = arr.iter().sum();

    for (i, v) in arr.iter().enumerate() {
        let sum_left: i32 = arr[..i].iter().sum();
        let sum_right = total - sum_left - v;
        // println!("im: {} sum_left: {} sum_right: {}", i, sum_left, sum_right);

        if sum_left == sum_right {
            return yes_answer;
        }
    }

    no_answer
}
