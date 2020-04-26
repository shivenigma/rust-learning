use std::io;
fn main() {
    let mut str = String::new();
    let str1 = str;
    io::stdin().read_line(&mut str);
}
