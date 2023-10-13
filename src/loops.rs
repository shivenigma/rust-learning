pub fn start() {
    loop_using_for();
    loop_using_loop();
    loop_using_while();
}
fn loop_using_for() {
    for number in 0..6 {
        println!("Printed in for, iteration {}", number);
    }
}
fn loop_using_loop() {
    let mut index: i8 = 0;
    loop {
        println!("Printed using loop, iteration {}", index);
        index = index + 1;
        if index > 5 {
            break;
        }
    }
}
fn loop_using_while() {
    let mut index: i8 = 0;
    while index <= 5 {
        println!("Printed using while, iteration {}", index);
        index = index + 1;
    }
}
