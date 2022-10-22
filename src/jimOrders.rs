/* cSpell:disable */
fn main() {
    assert_eq!(
        Vec::from([4, 5, 3, 1, 2]),
        jimOrders(&[
            Vec::from([8, 3]),
            Vec::from([5, 6]),
            Vec::from([6, 2]),
            Vec::from([2, 3]),
            Vec::from([4, 3])
        ])
    );
    assert_eq!(
        Vec::from([1, 2, 3]),
        jimOrders(&[Vec::from([1, 3]), Vec::from([2, 3]), Vec::from([3, 3]),])
    );
    assert_eq!(
        Vec::from([4, 2, 5, 1, 3]),
        jimOrders(&[
            Vec::from([8, 1]),
            Vec::from([4, 2]),
            Vec::from([5, 6]),
            Vec::from([3, 1]),
            Vec::from([4, 3]),
        ])
    );

    println!("Success!")
}

#[allow(non_snake_case)]
fn jimOrders(orders: &[Vec<i32>]) -> Vec<i32> {
    let mut ord_st = Vec::new();

    for (i, o) in orders.iter().enumerate() {
        let st = o[0] + o[1];
        ord_st.push(Vec::from([st, i as i32 + 1]))
    }

    ord_st.sort();

    let mut answer = Vec::new();

    for on in &ord_st {
        answer.push(on[1])
    }

    // println!("{:?}", answer);

    answer
}
