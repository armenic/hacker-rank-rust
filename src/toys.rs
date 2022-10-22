/* cSpell:disable */
fn main() {
    assert_eq!(2, toys(&[1, 2, 3, 4, 5, 10, 11, 12, 13]));
    assert_eq!(4, toys(&[1, 2, 3, 21, 7, 12, 14, 21]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn toys(w: &[i32]) -> i32 {
    let mut sorted_w = Vec::from(w);
    sorted_w.sort();
    let mut min_v = sorted_w.first().unwrap();
    let mut n_cont = 1;

    for v in &sorted_w {
        if *v > (min_v + 4) {
            n_cont = n_cont + 1;
            min_v = v;
        }
        // println!("{v} {min_v} {n_cont}");
    }

    n_cont
}
