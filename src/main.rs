fn main() {
    let str = String::from("Vignesh");
    let num = calculate_len(&str);
    println!("Str {}, length is {}", str, num);
}
fn calculate_len(str: &String) -> usize {
    str.insert(3, 'v');
    str.len()
}
