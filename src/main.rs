fn main() {
    let mut str = String::from("Vignesh");
    let str1 = &mut str;
    let str2 = &mut str;
    println!("String {} {}", str1, str2);
    let num = calculate_len(&mut str);
    println!("Str {}, length is {}", str, num);
}
fn calculate_len(str: &mut String) -> usize {
    str.insert(3, 'v');
    str.len()
}
