use std::io;
pub fn another_function(name: String) {
    println!("Hello {}", name);
}
pub fn block_function() {
    let x = 5;
    let y = {
        let x = 4;
        x +1
    };
    println!("x is {}, y is {}", x, y);
}
fn print_with_title(name: String, gender: char) {
    let mut title = if gender == 'm' {
        "Mr"
    } else if gender == 'f' {
        "Ms"
    } else {
        "Ind"
    };
    println!("Hello, {} {}", title, name);
}
pub fn ask_name_and_wish() {
    println!("Hi Please Enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Please enter a valid name");
    println!("Please enter your gender as m or f or o (O for non binary people)?");
    let mut gender_input = String::new();
    io::stdin().read_line(&mut gender_input).expect("Please enter either m or f or o (o for non binary people)");
    let gender: Vec<char> = gender_input.chars().collect();
    print_with_title(name, gender.get(0));
}
