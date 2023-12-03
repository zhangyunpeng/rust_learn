use rand::Rng;
use rand::distributions::{Distribution, Uniform};
pub fn rand_number1() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen::<u8>()
}

pub fn rand_number_range() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range::<u8, _>(0..10)
}

// Uniform 可以用于生成均匀分布uniform distribution的随机数。当需要在同一个范围内重复生成随机数时，该方法虽然和之前的方法效果一样，但会更快一些
pub fn rand_number_range2() {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("{}", throw);
        if throw == 6 {
            break;
        }
    }
}