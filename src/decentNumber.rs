/* cSpell:disable */
fn main() {
    assert_eq!("-1", decentNumber(1));
    assert_eq!("-1", decentNumber(2));
    assert_eq!("555", decentNumber(3));
    assert_eq!("-1", decentNumber(4));
    assert_eq!("33333", decentNumber(5));
    assert_eq!("55555533333", decentNumber(11));
    assert_eq!("555555555555555", decentNumber(15));
    assert_eq!("5553333333333", decentNumber(13));

    println!("Success!")
}

#[allow(non_snake_case)]
fn decentNumber(n: i32) -> String {
    let answer;
    if [1, 2].contains(&n) {
        answer = "-1".to_string();
        println!("{answer}");
        return answer;
    }

    // Deal with fives
    if n % 3 == 0 {
        answer = "5".repeat(n as usize);
        println!("{answer}");
        return answer;
    }

    let mut n_mut = n;
    let mut i = 0;
    loop {
        i = i + 1;
        n_mut = n_mut - 5;

        if n_mut % 3 == 0 {
            let s_01 = "5".repeat(n_mut as usize);
            let s_02 = "3".repeat((i * 5) as usize);
            answer = format!("{s_01}{s_02}");
            println!("{answer}");
            return answer;
        }
        if n_mut < 3 {
            break;
        }
    }

    // Deal with 3
    if n % 5 == 0 {
        answer = "3".repeat(n as usize);
        println!("{answer}");
        return answer;
    }

    answer = "-1".to_string();
    println!("{answer}");
    answer
}
