/**
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */
pub mod pig_latin {
    use std::fmt::format;
    use std::io;
    use std::ops::Add;

    pub fn start() {
        println!("Enter your text");
        let mut str = String::new();
        let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        io::stdin().read_line(&mut str)
            .expect("Failed to read line");
        let mut latin: Vec<String> = str
            .split_whitespace()
            .map(|word| {
                if let Some(first_char) = word.chars().next() {
                    if  vowels.contains(&first_char) {
                        format!("{}hay", word)
                    } else {
                        format!("{}{}ay", &word[1..], first_char)
                    }
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<String>>();
        println!("Pig Latin: {:?}]", latin.join(" "));
    }
}
