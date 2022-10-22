/* cSpell:disable */
fn main() {
    assert_eq!(3, findMedian(&[5, 3, 1, 2, 4]));
    assert_eq!(3, findMedian(&[0, 1, 2, 4, 6, 5, 3]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn findMedian(arr: &[i32]) -> i32 {
    let mut sorted_arr = Vec::from(arr);
    sorted_arr.sort();

    let middle = arr.len() / 2;

    let answer = sorted_arr[middle];

    answer
}
