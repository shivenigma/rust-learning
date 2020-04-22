pub mod functions;
use std::io;
fn main() {
    let mut name = String::new();
    println!("Please enter your name");
    io::stdin().read_line(&mut name);
    functions::another_function(name);
}
