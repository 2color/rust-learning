// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
use std::fmt::Write;

fn main() {
    println!("first: {}", pig_latin("first"));
    println!("apple: {}", pig_latin("apple"));
    println!("cheese: {}", pig_latin("cheese"));
    println!("elegant: {}", pig_latin("elegant"));
}

fn pig_latin(s: &str) -> String {
    let first_char = s.chars().next().expect("valid first char");
    if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) {
        format!("{}-hay", s)
    } else {
        let mut word = String::new();
        for c in s.chars().skip(1) {
            word.push(c);
        }
        write!(word, "-{}ay", first_char).unwrap();
        word
    }
}
