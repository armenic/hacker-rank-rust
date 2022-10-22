/* cSpell:disable */
fn main() {
    assert_eq!(7, maximizingXor(11, 12));
    assert_eq!(7, maximizingXor(10, 15));
    assert_eq!(127, maximizingXor(11, 100));

    println!("Success!")
}

#[allow(non_snake_case)]
fn maximizingXor(l: i32, r: i32) -> i32 {
    let mut answer = 0;

    for x in l..=r {
        for y in l..=r {
            if y > x {
                let xor = x ^ y;

                // println!("{x} {y} {xor}");

                if xor > answer {
                    answer = xor
                }
            }
        }
    }

    answer
}
