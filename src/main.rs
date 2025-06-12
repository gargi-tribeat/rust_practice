
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

    printLineAndAddSpace();

    // exampe usage of median and mode function
    let numbers = vec![1, 6, 2, 3, 4, 5, 5, 5, 2];
    let (median, modes) = stats::median_and_mode(numbers);
    println!("Median: {}, Modes: {:?}", median, modes);

    printLineAndAddSpace();

    // Example usage of pig latin conversion
    let words = vec!["first", "apple", "banana", "orange"];
    for word in words {
        let pig_latin = pig_latin::pig_latin_conversion(word);
        println!("Pig Latin of '{}': {}", word, pig_latin);
    }

    printLineAndAddSpace();

    // employees in company
    employees_in_company::employees_in_company();
    println!("End of solutions to Vector and Hashmap problems!");
}

fn printLineAndAddSpace(){
    println!("------------------------------------------------");
    println!("\n\n");
}


