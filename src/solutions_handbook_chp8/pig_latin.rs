// This module provides a function to convert a string into Pig Latin.
pub fn pig_latin_conversion(input: &str) -> String {
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