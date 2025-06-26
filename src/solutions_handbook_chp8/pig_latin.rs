// This module provides a function to convert a string into Pig Latin.

pub fn pig_latin_conversion(input: &str) -> String {
    if !input.is_ascii() {
       // check if input is Hindi
       if input.chars().all(|c| is_hindi(c)) {
            return pig_latin_for_hindi(input);
        }
        // If input is not ASCII and not Hindi, return an error message
        return "Input must be either ASCII or Hindi.".to_string();
    }
    input
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().// Get the first character of the word
                                  next(). // Use `next()` to get the first character
                                  unwrap(); // Unwrap the Option to get the character, assuming the word is not empty
            if "aeiouAEIOU".contains(first_char) {
                format!("{}-hay", word)
            } else {
                let mut chars = word.chars(); // Create an iterator over the characters of the word
                let first_consonant = chars.next().unwrap();
                format!("{}-{}ay", chars.as_str(), first_consonant)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn pig_latin_for_hindi(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().next().unwrap();
            if "अआइईउऊऋ".contains(first_char) {
                format!("{}-है", word)
            } else {
                let mut chars = word.chars();
                let first_consonant = chars.next().unwrap();
                format!("{}-{}य", chars.as_str(), first_consonant)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_hindi(c: char) -> bool {
    // Check if the character is a Hindi character
    c >= '\u{0900}' && c <= '\u{097F}'
}