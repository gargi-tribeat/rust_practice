   // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
   //  for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department,  sorted alphabetically.
   use std::collections::HashMap;
   use std::io::{self, Write};

   pub fn employees_in_company() {
   
      let mut company: HashMap<String, Vec<String>> = HashMap::new();
   
      loop {
         print!("Enter command (Add <Name> to <Department> or List <Department> or List All): ");
         io::stdout().flush().unwrap(); // Ensure prompt is printed before reading input
   
         let mut input = String::new();
         io::stdin().read_line(&mut input).unwrap();
         let input = input.trim();
   
         if input.starts_with("Add") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            println!("{:?}", parts);
            if parts.len() == 4 && parts[2] == "to" {
               let name = parts[1].to_string();
               let department = parts[3].to_string();
               company.entry(department).or_default().push(name);
            } else {
               println!("Invalid command format. Use 'Add <Name> to <Department>'.");
            }
         } else if input.starts_with("List") {
               let parts: Vec<&str> = input.split_whitespace().collect();
               println!("{:?}", parts);
               if parts.len() == 2 && parts[1] == "All" {
                  for (dept, employees) in &company {
                     println!("{}: {:?}", dept, employees);
                  }
               } else if parts.len() == 2 && parts[0] == "List" {
                  if parts[1].is_empty() {
                     println!("Please specify a department.");
                     continue;
                  }
                  let department = parts[1].to_string();
                  if let Some(employees) = company.get(&department) {
                     println!("{}: {:?}", department, employees);
                  } else {
                     println!("No employees found in department '{}'.", department);
                  }
               }
               else {
                  println!("Invalid command format. Use 'List <Department>' or 'List All' or 'exit'.");
               }
         } else if input.to_lowercase() == "exit" {
               break;
         } else {
               println!("Unknown command. Please try again.");
         }
      }
      println!("Exiting employee management system.");
   }