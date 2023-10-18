pub mod median {
    use std::io;
    pub fn start() {
        println!("Enter the length of your collection");
        let mut length: String = String::new();
        io::stdin().read_line(&mut length)
            .expect("Failed to read length");
        let length: u32 = match length.trim().parse() {
            Ok(num: u32) => num,
            Err(_) => {
            println!("Invalid number");
            continue;
        }
        };
    }
}
