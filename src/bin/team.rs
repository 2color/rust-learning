use std::collections::HashMap;
use std::io;

// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn main() {
    println!(
        "Add new employee to department in the following format:\n\
        \"Add Sally to Engineering\"\n\
        Or list employees in a department:\n\
        \"list Engineering\"\n\
        or list all employees:\n\
        \"list\"\n\
        "
    );
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    // let mut departments = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parsed: Vec<&str> = input.trim().split(' ').collect();
        match parsed.len() {
            1 => {
                if parsed[0].len() == 0 {
                    continue;
                }
                for (dep, employees) in &mut departments {
                    employees.sort();
                    println!("Department: {}, employees: {:?}", dep, employees)
                }
            }
            2 => match departments.get_mut(parsed[1]) {
                Some(employees) => {
                    employees.sort();
                    println!("employees: {:?}", employees)
                }
                None => {
                    println!("no employees in department: {}", parsed[1]);
                }
            },
            4 => {
                let name = parsed[1];
                let department_name = parsed[3];
                let team = departments
                    .entry(department_name.to_string())
                    .or_insert(Vec::new());
                team.push(name.to_string());
                println!("Added {} to {}", name, department_name);
            }
            _ => {
                println!("invalid input. Try again");
                continue;
            }
        }
    }
}
