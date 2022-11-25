/* cSpell:disable */

fn main() {
    assert_eq!("YES 1", separateNumbers("1234"));
    // assert_eq!("YES 2", separateNumbers("234567"));
    // assert_eq!("NO", separateNumbers("13"));
    // assert_eq!("NO", separateNumbers("1"));
    // assert_eq!("NO", separateNumbers("010203"));
    // assert_eq!("YES 9", separateNumbers("91011"));
    // assert_eq!("YES 99", separateNumbers("99100"));
    // assert_eq!("YES 999", separateNumbers("99910001001"));
    // assert_eq!("YES 7", separateNumbers("7891011"));
    // assert_eq!("YES 98", separateNumbers("9899100"));
    // assert_eq!("NO", separateNumbers("101103"));
    // assert_eq!("NO", separateNumbers("999100010001"));
    // assert_eq!("NO", separateNumbers("00000000000000000000000000000000"));
    // assert_eq!("NO", separateNumbers("11111111111111111111111111111111"));
    // assert_eq!(
    //     "YES 1000",
    //     separateNumbers("10001001100210031004100510061007")
    // );
}

#[allow(non_snake_case)]
// fn separateNumbers(s: &str) {
fn separateNumbers(s: &str) -> String {
    if s.len() == 1 {
        return "NO".to_string();
        // println!("{}", "NO".to_string());
        // return;
    }
    if s.chars().next().unwrap() == '0' {
        return "NO".to_string();
        // println!("{}", "NO".to_string());
        // return;
    }

    let mut f_c = String::from("");
    let mut n_d = 1;
    let mut not_found = false;

    loop {
        if n_d > s.len() - 1 {
            break;
        }
        let mut f_c_v = Vec::new();

        let mut s_c = s.chars();
        for _ in 0..n_d {
            f_c_v.push(s_c.next().unwrap().to_string());
        }

        f_c = f_c_v.join("");
        let mut n_01: i128 = f_c.parse().unwrap();

        loop {
            // println!("s_c start {:?} {}", s_c, s_c.clone().count());
            if s_c.clone().count() == 0 {
                not_found = false;
                break;
            }
            // println!("n_01 {}", n_01);
            n_01 += 1;
            let c_to_advance = n_01.to_string();
            let n_c = n_01.to_string();
            let new_s: String = s_c.clone().collect();
            // println!("{} {}", new_s, n_c);
            let i_n_c = new_s.find(&n_c);
            if i_n_c != Some(0) {
                not_found = true;
                break;
            }

            // Advance as many times as the number of digits
            // println!("c_to_advance {}", c_to_advance.len());

            for _ in 0..(c_to_advance.len()) {
                s_c.next();
                // println!("advancing {:?}", s_c);
            }
        }

        if !not_found {
            break;
        }
        n_d += 1;
    }

    let res = if not_found {
        "NO".to_string()
    } else {
        format!("YES {}", f_c)
    };

    // println!("{}", res);

    return res;
}
