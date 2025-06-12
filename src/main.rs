
// This is a simple Rust program that demonstrates the use of vectors and hashmaps
mod solutions_handbook_chp8;

use solutions_handbook_chp8::stats;
use solutions_handbook_chp8::pig_latin;

fn main() {
   solutions_handbook_chp8();
}


fn solutions_handbook_chp8(){
    let s = String::from("Solutions to Vector and Hashmap problems!");
    println!("{s}");
    // exampe usage of median and mode function
    let numbers = vec![1, 6, 2, 3, 4, 5, 5, 5, 2];
    let (median, modes) = stats::median_and_mode(numbers);
    println!("Median: {}, Modes: {:?}", median, modes);

    // Example usage of pig latin conversion
    let words = vec!["first", "apple", "banana", "orange"];
    for word in words {
        let pig_latin = pig_latin::pig_latin_conversion(word);
        println!("Pig Latin of '{}': {}", word, pig_latin);
    }
}



