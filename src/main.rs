fn main() {
    let mut str = String::from("Vignesh");
    {
        let str2 = &mut str;
        println!("String in separate scope {}", str2);
    }
    let str1 = &mut str;
    println!("String in main scope{}", str1);
    let num = calculate_len(&mut str);
    println!("Str {}, length is {}", str, num);
}
fn calculate_len(str: &mut String) -> usize {
    str.insert(3, 'v');
    str.len()
}
