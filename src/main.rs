//cargo run -- arg1 arg2
fn main() {
    let mut args = std::env::args().skip(1);

    println!("{:?}", args.next().unwrap());
}
