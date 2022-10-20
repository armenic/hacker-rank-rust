/* cSpell:disable */
fn main() {
    assert_eq!(Vec::from([1, 4]), icecreamParlor(6, &[1, 3, 4, 5, 6]));
    assert_eq!(Vec::from([4, 5]), icecreamParlor(6, &[6, 3, 4, 5, 1]));
    assert_eq!(Vec::from([1, 4]), icecreamParlor(4, &[1, 4, 5, 3, 2]));
    assert_eq!(Vec::from([1, 2]), icecreamParlor(4, &[2, 2, 4, 3]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn icecreamParlor(m: i32, arr: &[i32]) -> Vec<i32> {
    /* cSpell:enable */

    // There is only a unique solution.
    // - Two numbers smaller than the money add up to money

    let mut final_ = Vec::new();
    for (i, v) in arr.iter().enumerate() {
        let r = m - v;
        final_ = find_rest(arr, final_, i, r);
        if final_.len() == 2 {
            return final_;
        }
    }
    return final_;
}

fn find_rest(arr: &[i32], mut final_: Vec<i32>, i: usize, r: i32) -> Vec<i32> {
    if r > 0 {
        for (ii, vv) in arr.iter().enumerate() {
            // println!("{} {} {}", v, r, vv);
            if vv == &r && ii != i {
                final_.push(i as i32 + 1);
                final_.push(ii as i32 + 1);
                return final_;
            }
        }
    }
    return final_;
}
