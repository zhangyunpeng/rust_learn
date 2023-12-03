use rust_learn::algorithm::random_number::funcs;

#[test]
fn test_random_number1() {
    println!("{}", funcs::rand_number1());
}
#[test]
fn test_rand_number_range() {
    println!("{}", funcs::rand_number_range())
}
#[test]
fn test_rand_number_range2() {
    funcs::rand_number_range2()
}