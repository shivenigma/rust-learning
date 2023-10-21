/**
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */
pub mod pig_latin {
    use std::io;
    use std::ops::Add;

    pub fn start() {
        println!("Enter your text");
        let mut str = String::new();
        let mut latin: Vec<String> = Vec::new();
        let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        io::stdin().read_line(&mut str)
            .expect("Failed to read line");
        for word in str.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                if  vowels.contains(&first_char) {
                    latin.push(word.to_string().add("hay"));
                } else {
                    word.to_string().push(first_char);
                    word.to_string().remove(0);
                    latin.push(word.to_string().add("ay"));
                }
            }
        }
        println!("Pig Latin: {:?}]", latin.join(" "));
    }
}
