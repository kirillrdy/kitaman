use std::env;
fn main() {
    println!("Hello kitaman");
    let command = env::args().nth(1);
    println!("{:?}", command);
}
