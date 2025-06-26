   use std::collections::HashMap;
   use std::io::{self, Write};

   pub fn employees_in_company() {
      let mut company: HashMap<String, Vec<String>> = HashMap::new();
   
      loop {
         print!("Enter command (Add <Name> to <Department> or List <Department> or List All): ");
         io::stdout().flush().unwrap();
         let mut input = String::new();
         io::stdin().read_line(&mut input).unwrap();
         let input = input.trim();
         let parts: Vec<&str> = input.split_whitespace().collect();
         match parts[0] {
            "Add" if parts.len() == 4 && parts[2] == "to" => {
               let name = parts[1].to_string();
               let department = parts[3].to_string();
               company.entry(department).or_default().push(name);
            }
            "List" if parts.len() == 2 && parts[1] == "All" => {
               for (dept, employees) in &company {
                  println!("{}: {:?}", dept, employees);
               }
            }
            "List" if parts.len() == 2 => {
               let department = parts[1].to_string();
               if let Some(employees) = company.get(&department) {
                  println!("{}: {:?}", department, employees);
               } else {
                  println!("No employees found in department '{}'.", department);
               }
            }
            "exit" => break,
            _ => println!("Invalid command format. Use 'Add <Name> to <Department>' or 'List <Department>' or 'List All' or 'exit'."),
         }
      }
      println!("Exiting employee management system.");
   }