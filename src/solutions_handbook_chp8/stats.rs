use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
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
    let max_occurrence = occurrences.values().cloned().max().unwrap_or(0);
    let modes: Vec<i32> = occurrences.into_iter()
        .filter(|&(_, count)| count == max_occurrence)
        .map(|(num, _)| num)
        .collect();

    (median, modes)
}