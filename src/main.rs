
// This is a simple Rust program that demonstrates the use of vectors and hashmaps
mod solutions_handbook_chp8;

use solutions_handbook_chp8::stats;
use solutions_handbook_chp8::pig_latin;
use solutions_handbook_chp8::employees_in_company;

fn main() {
   solutions_handbook_chp8();
}


fn solutions_handbook_chp8(){
    println!("Solutions to Vector and Hashmap problems from the Rust Solutions Handbook Chapter 8");

    print_line_and_add_space();

    // exampe usage of median and mode function
    let numbers = vec![1, 6, 2, 3, 4, 5, 5, 5, 2];
    let (median, modes) = stats::median_and_mode(numbers);
    println!("Median: {}, Modes: {:?}", median, modes);

    print_line_and_add_space();

    // Example usage of pig latin conversion
    let words = vec!["first apple", "apple mango", "banana orange", "orange yellow"]; 
    for word in words {
        let pig_latin = pig_latin::pig_latin_conversion(word);
        println!("Pig Latin of '{}': {}", word, pig_latin);
    }
    let hindi_words = vec!["नमस्ते", "सुप्रभात", "शुभरात्रि", "आशी"];
    for word in hindi_words {
        let pig_latin = pig_latin::pig_latin_conversion(word);
        println!("Pig Latin of '{}': {}", word, pig_latin);
    }

    print_line_and_add_space();

    // employees in company
    employees_in_company::employees_in_company();
    println!("End of solutions to Vector and Hashmap problems!");
}

fn print_line_and_add_space(){
    println!("------------------------------------------------");
    println!("\n\n");
}


