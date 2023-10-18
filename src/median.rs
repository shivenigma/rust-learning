pub mod median {
    use std::io;
    use std::result::Result::*;
    use std::string::String;
    use std::vec::Vec;

    pub fn start() {
        println!("Enter the length of your collection");
        let mut length: String = String::new();
        io::stdin().read_line(&mut length)
            .expect("Failed to read length");
        let length: u32 = match length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                0
            }
        };
        if length > 0 {
            let mut vec: Vec<i32> = Vec::with_capacity(length as usize);
            for i in 0..length {
                println!("Enter item {}", i);
                let mut temp: String = String::new();
                io::stdin().read_line(&mut temp)
                    .expect("Failed to read length");
                match temp.trim().parse() {
                    Ok(num) => {
                        vec.push(num);
                    },
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };
            }
            vec.sort();
            let median = vec[vec.len()/2];
            println!("The median is: {}", median);
        }
    }
}
