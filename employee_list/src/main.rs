use std::{io::{self, Write}, collections::HashMap};

mod company {
    use std::collections::HashMap;    

    #[derive(Debug)]
    pub enum Operation {
        GetAll, 
        GetDepartment(String), // department :String:
        Add(String, String), // name: String , department: String
        Exit,
        Invalid,
    }

    #[derive(Debug)]
    pub struct Command {
        pub op: Operation,
    }

    impl Command {
        fn print_employees(employee_list: &Vec<String>) {
            let mut sorted_list = employee_list.clone();
            sorted_list.sort();
            for employee_name in sorted_list {
                println!("- {employee_name}");
            }
        }

        fn print_all_employees(department_list: &HashMap<String, Vec<String>>) {
            // temporary create a vector to sort HashMap by department name.
            let mut department_list_vector: Vec<(&String, &Vec<String>)> = department_list.iter().collect();
            department_list_vector.sort_by(|a, b| a.0.cmp(b.0));

            for (department, employee_list) in department_list_vector {
                println!("[{department}]");
                Self::print_employees(employee_list);
                println!("");
            }
        }

        fn print_department(department_list: &HashMap<String, Vec<String>>, department: String) {
            match department_list.get(&department) {
                Some(employee_list) => {
                    println!("[{department}]");
                    Self::print_employees(employee_list);
                    println!("");                
                },
                None => {
                    println!("{department} department is not exist");
                }
            }
        }

        fn add_employee(department_list: &mut HashMap<String, Vec<String>>, name: String, department: String) {
            let department = department_list.entry(department).or_insert(vec![]);
            department.push(name);
        }    

        pub fn execute(self, department_list: &mut HashMap<String, Vec<String>>) {
            match self.op {
                Operation::GetAll => Self::print_all_employees(department_list),
                Operation::GetDepartment(department) => Self::print_department(department_list, department),
                Operation::Add(name, department) => Self::add_employee(department_list, name, department),
                Operation::Invalid => {
                    println!("The input is invalid.")
                },
                _ => (),
            }
        }

        pub fn from(command: Vec<String>) -> Self {
            let first_word = command.get(0);
        
            match first_word {
                Some(word) => {
                    match &word[..] {
                        "Get" => Self::get_command(command),
                        "Add" => Self::add_command(command),
                        "exit" => Self { op: Operation::Exit },
                        _ => Self { op: Operation::Invalid },
                    }
                }
                None => Self {op: Operation::Invalid},
            }
        }

        // valid format1: vec!["Get",]
        // valid format2: vec!["Get", "Engineering"]
        fn is_valid_get_command(command: &Vec<String>) -> bool {
            // Here, we check only the number of words. 
            // The actual check would be a more complex condition.
            match command.len()  {
                1 | 2 => true,
                _ => false,
            }
        }

        fn get_command(command: Vec<String>) -> Self {
            if !Self::is_valid_get_command(&command) {
                return Self { op: Operation::Invalid }
            }

            // command example: vec!["Get", "Engineering"]        
            let department = command.get(1);
            match department {
                Some(department) => Self {
                    op: Operation::GetDepartment(department.to_string())
                },
                None => Self { op: Operation::GetAll },
            }
        }

        // valid format: vec!["Add", "Amir", "to", "Sales"]
        fn is_valid_add_command(command: &Vec<String>) -> bool {
            // Here, we check only the number of words. 
            // The actual check would be a more complex condition.
            match command.len()  {
                4 => true,
                _ => false,
            }
        }    

        fn add_command(command: Vec<String>) -> Self {
            if !Self::is_valid_add_command(&command) {
                return Self { op: Operation::Invalid }
            }

            // command example: vec!["Add", "Amir", "to", "Sales"]   
            let name = command.get(1).unwrap();
            let department = command.get(3).unwrap();
            Self {
                op: Operation::Add(
                    name.to_string(),
                    department.to_string(),
                )
            }
        }
    }
}

// return string vector from stdin input
fn get_string_vector_from_input() -> Vec<String> {
    let mut input = String::new();

    print!("> ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin().read_line(&mut input)
        .expect("failed to read_line");

    let mut word_list: Vec<String> = Vec::new();
    for word in input.split_whitespace() {
        word_list.push(word.to_string());
    }

    word_list
}

fn main() {
    let mut departments_list: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = get_string_vector_from_input();
        let command = company::Command::from(input);

        if let company::Operation::Exit = command.op { break; }
        command.execute(&mut departments_list);
    }
}
