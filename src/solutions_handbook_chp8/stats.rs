use std::collections::HashMap;

pub fn median_and_mode(vector: Vec<i32>) -> (f64, Vec<i32>) {
    // Sort the vector to find the median
    let mut sorted_vector = vector.clone();
    sorted_vector.sort();

    // Calculate the median
    let len = sorted_vector.len();
    let median = if len % 2 == 0 {
        (sorted_vector[len / 2 - 1] + sorted_vector[len / 2]) as f64 / 2.0
    } else {
        sorted_vector[len / 2] as f64 // not sure why if and else have to have same types 
    };

    // Calculate the mode using a hash map
    let mut occurrences = HashMap::new();
    for &num in &sorted_vector {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    // Find the maximum occurrence(s)
    let max_occurrence = occurrences.values() // Get the values of the hash map
                                    .cloned() // Clone the values to avoid borrowing issues
                                    .max() // Find the maximum occurrence count
                                    .unwrap_or(0); // Use `unwrap_or(0)` to handle the case of an empty vector

    let modes: Vec<i32> = occurrences.into_iter() // Convert the hash map into an iterator of (key, value) pairs
        .filter(|&(_, count)| count == max_occurrence) // Filter the pairs to keep only those with the maximum occurrence count
        .map(|(num, _)| num) // Map the filtered pairs to just the numbers (keys)
        .collect(); // Collect the results into a vector of modes

    (median, modes)
}