use rust_learn::self_vec::svec::*;
#[test]
fn self_vec() {
    let mut v = SVec::<i32>::new();
    v.push(1);
    v.push(2);
    v.push(3);
    do_eq(&v, vec![1, 2, 3]);

    v.pop();
    do_eq(&v, vec![1, 2]);

    v.push(3);
    v.insert(1, 4);
    do_eq(&v, vec![1, 4, 2, 3]);

    v.remove(1);
    do_eq(&v, vec![1, 2, 3]);

    assert_eq!(v.last(), Some(&3));
    assert_eq!(v.first(), Some(&1));
    assert_eq!(3, v.len());
    assert_eq!(&v[..1], vec![1]);
    do_eq(&v, vec![1, 2, 3]);

    for item in v.iter_mut() {
        *item += 1;
    }
    do_eq(&v, vec![2,3,4]);

}

fn do_eq(v: &SVec<i32>, v_eq: Vec<i32>){
    let mut v_helper = Vec::new();
    for item in v.iter() {
        v_helper.push(*item);
    }
    assert_eq!(v_helper, v_eq);
}