fn main() {
    assert_eq!(44, marcsCakewalk(&[5, 10, 7]));
    assert_eq!(11, marcsCakewalk(&[1, 3, 2]));
    assert_eq!(79, marcsCakewalk(&[7, 4, 9, 6]));
    assert_eq!(
        59715404338867,
        marcsCakewalk(&[
            819, 701, 578, 403, 50, 400, 983, 665, 510, 523, 696, 532, 51, 449, 333, 234, 958, 460,
            277, 347, 950, 53, 123, 227, 646, 190, 938, 61, 409, 110, 61, 178, 659, 989, 625, 237,
            944, 550, 954, 439
        ])
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn marcsCakewalk(calorie: &[i32]) -> i64 {
    let base: i64 = 2;
    let mut new_cal: Vec<i64> = Vec::new();
    for c in calorie {
        new_cal.push(c.clone() as i64);
    }

    // Need reverse sorting, i.e. descending order
    new_cal.sort();
    new_cal.reverse();

    let miles: i64 = new_cal
        .iter()
        .enumerate()
        .map(|pair| base.pow(pair.0 as u32) * pair.1)
        .sum();

    miles
}
