use rust_learn::self_sort::quick::sort;

#[test]
fn quick_sort() {
    let mut v = vec![3,2,1];
    sort(&mut v);
    assert_eq!(v, vec![1,2,3]);
}