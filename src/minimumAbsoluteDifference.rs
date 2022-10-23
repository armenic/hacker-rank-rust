/* cSpell:disable */
fn main() {
    assert_eq!(2, minimumAbsoluteDifference(&[-2, 2, 4]));
    assert_eq!(3, minimumAbsoluteDifference(&[3, -7, 0]));
    assert_eq!(3, minimumAbsoluteDifference(&[1, -3, 71, 68, 17]));
    assert_eq!(
        1,
        minimumAbsoluteDifference(&[-59, -36, -13, 1, -53, -92, -2, -96, -54, 75])
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn minimumAbsoluteDifference(arr: &[i32]) -> i32 {
    let mut sorted_a = Vec::from(arr);
    sorted_a.sort();

    let mut answer = (sorted_a.first().unwrap() - sorted_a.last().unwrap()).abs();

    for (i, v) in sorted_a.iter().enumerate() {
        if i == arr.len() - 1 {
            break;
        }

        let new_diff = (v - sorted_a[i + 1]).abs();
        if new_diff < answer {
            answer = new_diff
        }
    }

    answer
}
