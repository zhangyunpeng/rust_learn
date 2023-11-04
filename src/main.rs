use learn_macro::func_log;

#[func_log]
fn helper() {
    println!("running helper")
}
fn main() {
    helper();
}
