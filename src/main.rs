mod guess;
use crate::guess::guess as gus;
fn main() {
    gus::start();
    // Difference between different print macros

    print!("I just print this {}", "string without any new lines");
    print!("<--Next lines are appended to previous one without new line character");

    print!("\n----------------------------------------------------------------\n");

    println!("I just print this {}", "string with new lines");
    print!("Next statement is automatically printed into next lines");

    print!("\n----------------------------------------------------------------\n");

    eprint!("This is an {}", "error without new line");
    eprint!(" Hard to read");

    print!("\n----------------------------------------------------------------\n");

    eprintln!("This is an {}", "error with new line");
    eprintln!(" Much easier to read");

    let mut v: Vec<i32> = Vec::new();
    v.push(32);
    for i in &v {
        println!("{i}");
    }
    let v1 = vec!["test", 1];
}
