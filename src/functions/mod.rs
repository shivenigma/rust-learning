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
