use std::env;
fn main() {
    println!("Hello, world!");
    let path = env::var("PATHIO").unwrap_or_else(|error|{

    });
    println!("{}", path)
}
