use std::io;

fn main() {
    loop {
        println!("Temperature converter\n");
        println!("Select your conversion method: \n1. Fahrenheit -> Celsius \n2. Celsius -> Fahrenheit \n\n Press any other key to exit the program");
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Invalid Choice");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        if selection == 2 {
            celsius_to_fahrenheit();
        } else if selection == 1 {
            fahrenheit_to_celsius();
        } else {
            break;
        }
    }
}
fn celsius_to_fahrenheit() {
    println!("Enter your temperature in Celsius");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to read temperature");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Enter a number");

    println!("You entered {temperature}C.");
    let temperature: f64 = (temperature * 1.8) + 32.0;
    println!("It is equal to {temperature} F");
}

fn fahrenheit_to_celsius() {
    println!("Enter your temperature in Fahrenheit");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to read temperature");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Enter a number");

    println!("You entered {temperature}F.");
    let temperature: f64 = (temperature - 32.0)/1.8;
    println!("It is equal to {temperature}C");
}
