fn main() {
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
}
