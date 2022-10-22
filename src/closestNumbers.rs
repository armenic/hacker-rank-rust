/* cSpell:disable */
fn main() {
    assert_eq!(
        Vec::from([-20, 30]),
        closestNumbers(&[
            -20, -3916237, -357920, -3620601, 7374819, -7330761, 30, 6246457, -6461594, 266854
        ])
    );
    assert_eq!(
        Vec::from([-520, -470, -20, 30]),
        closestNumbers(&[
            -20, -3916237, -357920, -3620601, 7374819, -7330761, 30, 6246457, -6461594, 266854,
            -520, -470
        ])
    );
    assert_eq!(Vec::from([2, 3, 3, 4, 4, 5]), closestNumbers(&[5, 4, 3, 2]));
    assert_eq!(
        Vec::from([1, 2, 2, 3, 3, 4, 4, 5]),
        closestNumbers(&[5, 2, 3, 4, 1])
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn closestNumbers(arr: &[i32]) -> Vec<i32> {
    let mut sorted_arr = Vec::from(arr);
    sorted_arr.sort();

    // println!("{:?}", sorted_arr);

    let mut min_diff = sorted_arr.last().unwrap() - sorted_arr.first().unwrap();
    let mut answer: Vec<i32> = Vec::new();

    for (i, a) in sorted_arr.iter().enumerate() {
        if i == sorted_arr.len() - 1 {
            break;
        }

        let next_item = sorted_arr[i + 1];
        let diff = next_item - a;

        if diff < min_diff {
            min_diff = diff;
            answer.clear();
            answer.extend_from_slice(&[*a, next_item]);
        } else if diff == min_diff {
            answer.extend_from_slice(&[*a, next_item]);
        }
    }

    // println!("{} {:?}", min_diff, answer);

    answer
}
