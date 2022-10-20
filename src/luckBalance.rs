/* cSpell:disable */
fn main() {
    assert_eq!(
        10,
        luckBalance(
            2,
            &[Vec::from([5, 1]), Vec::from([1, 1]), Vec::from([4, 0])]
        )
    );
    assert_eq!(
        8,
        luckBalance(
            1,
            &[Vec::from([5, 1]), Vec::from([1, 1]), Vec::from([4, 0])]
        )
    );
    assert_eq!(
        8,
        luckBalance(
            1,
            &[Vec::from([5, 1]), Vec::from([1, 1]), Vec::from([4, 0])]
        )
    );
    assert_eq!(
        29,
        luckBalance(
            3,
            &[
                Vec::from([5, 1]),
                Vec::from([2, 1]),
                Vec::from([1, 1]),
                Vec::from([8, 1]),
                Vec::from([10, 0]),
                Vec::from([5, 0])
            ]
        )
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn luckBalance(k: i32, contests: &[Vec<i32>]) -> i32 {
    let mut imp_n = 0;
    let mut max_luck = 0;
    let mut imp_lucks = Vec::new();

    for c in contests {
        max_luck = max_luck + c[0];
        imp_n = imp_n + c[1];
        if c[1] == 1 {
            imp_lucks.push(c[0])
        }
    }

    imp_lucks.sort();

    let n_to_lose: usize = (imp_n - k) as usize;

    // println!("{imp_n} {k} {max_luck} {:?}", imp_lucks);

    if imp_n <= k {
        return max_luck;
    } else {
        let score_to_lose: i32 = imp_lucks[0..n_to_lose].iter().sum();
        // Subtract twice since our max score includes the imporant scores
        return max_luck - 2 * score_to_lose;
    }
}
