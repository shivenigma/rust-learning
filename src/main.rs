fn main() {
    let mut str = String::from("Vignesh");
    let num = calculate_len(&mut str);
    println!("Str {}, length is {}", str, num);
}
fn calculate_len(str: &mut String) -> usize {
    str.insert(3, 'v');
    str.len()
}
