use std::collections::HashMap;

/* cSpell:disable */
fn main() {
    assert_eq!(
        Vec::from([4, 6]),
        missingNumbers(&[7, 2, 5, 3, 5, 3], &[7, 2, 5, 4, 6, 3, 5, 3])
    );
    assert_eq!(
        Vec::from([204, 205, 206]),
        missingNumbers(
            &[203, 204, 205, 206, 207, 208, 203, 204, 205, 206],
            &[203, 204, 204, 205, 206, 207, 205, 208, 203, 206, 205, 206, 204]
        )
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn missingNumbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut dic_01 = HashMap::new();
    for a in arr {
        let count = dic_01.entry(a).or_insert(0);
        *count += 1;
    }
    let mut dic_02 = HashMap::new();
    for b in brr {
        let count = dic_02.entry(*b).or_insert(0);
        *count += 1;
    }

    let mut answer = Vec::new();
    for (k, v) in dic_02.iter() {
        let cond_01 = dic_01.get_key_value(k);
        match cond_01 {
            Some(bv) => {
                if bv.1 != v {
                    answer.push(*k)
                }
            }
            None => answer.push(*k),
        }
    }

    answer.sort();

    // println!("{:#?} {:#?} {:#?}", dic_01, dic_02, answer);

    answer
}
