/* cSpell:disable */
fn main() {
    assert_eq!("YES", twoArrays(1, &[0, 0, 1], &[0, 2, 2]));
    assert_eq!("YES", twoArrays(1, &[1, 1], &[0, 2]));
    assert_eq!("YES", twoArrays(1, &[0, 1], &[0, 2]));
    assert_eq!("YES", twoArrays(10, &[2, 1, 3], &[7, 8, 9]));
    assert_eq!("NO", twoArrays(5, &[1, 2, 2, 1], &[3, 3, 3, 4]));

    println!("Success!")
}

#[allow(non_snake_case)]
fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    let yes_answer = "YES".to_string();
    let no_answer = "NO".to_string();

    let mut a_pr = Vec::new();
    let mut b_pr = Vec::new();

    for a in A {
        if a < &k {
            a_pr.push(a)
        }
    }

    for b in B {
        if b > &0 {
            b_pr.push(b)
        }
    }

    a_pr.sort();
    b_pr.sort();

    let mut used_b_i = Vec::new();

    if a_pr.len() == 0 {
        return yes_answer;
    }

    // println!("{:?}", a_pr);
    // println!("{:?}", b_pr);

    // Need to find out if there are enough items in B' to enhance A'
    for a in &a_pr {
        for (i, b) in b_pr.iter().enumerate() {
            if *a + *b >= k && !used_b_i.contains(&i) {
                used_b_i.push(i);
                break;
            }
        }
    }

    // println!("{:?}", used_b_i);

    if a_pr.len() <= used_b_i.len() {
        return yes_answer;
    }

    no_answer
}
