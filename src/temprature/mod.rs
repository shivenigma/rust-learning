fn celsius_to_fahrenheit(celsius: f64)-> f64 {
    return (celsius * 1.8) + 32.0;
}
fn fahrenheit_to_celsius(fahrenheit: f64)-> f64 {
    return (fahrenheit - 32.0) / 1.8
}
pub fn start() {
    let celsius = 36.7;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("The {} celsius is {} in fahrenheit", celsius, fahrenheit);
    let fahrenheit = 470.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("The {} fahrenheit is {} in celsius", fahrenheit, celsius);
}
