pub mod median {
    use std::collections::HashMap;
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
            let mut hash: HashMap<i32, i32> = HashMap::new();
            for i in 0..length {
                println!("Enter item {}", i);
                let mut temp: String = String::new();
                io::stdin().read_line(&mut temp)
                    .expect("Failed to read length");
                match temp.trim().parse() {
                    Ok(num) => {
                        vec.push(num);
                        let count = hash.entry(num).or_insert(0);
                        *count+=1;
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
            let mut max_key: i32 = 0;
            let mut max_val: i32 = 0;
            for (key, val) in hash.iter() {
                if val > &max_val {
                    max_key = *key;
                    max_val = *val;
                }
            }
            println!("The number {} has occured {} times", max_key, max_val);
        }
    }
}
