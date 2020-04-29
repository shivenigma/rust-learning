use std::io;
fn print_with_title(name: String, gender: u8) {
    let title = if gender == b'm' {
        "Mr"
    } else if gender == b'f' {
        "Ms"
    } else {
        "Ind"
    };
    println!("Hello, {} {}", title, name);
}
pub fn ask_name_and_greet() {
    println!("Hi Please Enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Please enter a valid name");
    println!("Please enter your gender as m or f or o (O for non binary people)");
    let mut gender_input = String::new();
    io::stdin().read_line(&mut gender_input).expect("Please enter either m or f or o (o for non binary people)");
    let gender = gender_input.as_bytes();
    println!("{:?}", gender[0]);
    print_with_title(name, gender[0]);
}
