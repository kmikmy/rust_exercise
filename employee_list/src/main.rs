use std::{io::{self, Write}, collections::HashMap};

#[derive(Debug)]
enum Operation {
    GetAll, 
    GetDepartment(String), // department :String:
    Add(String, String), // name: String , department: String
    Exit,
    Invalid,
}

#[derive(Debug)]
struct Command {
    op: Operation,
}

impl Command {
    fn print_employees(department_list: &mut HashMap<String, Vec<String>>) {
        for (department, employee_list) in department_list {
            println!("[{department}]");
            for employee_name in employee_list {
                println!("- {employee_name}");
            }
            println!("");
        }
    }

    fn print_department(department_list: &mut HashMap<String, Vec<String>>, department: String) {
        match department_list.get(&department) {
            Some(employee_list) => {
                println!("[{department}]");
                for employee_name in employee_list {
                    println!("- {employee_name}");
                }
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

    fn execute(self, department_list: &mut HashMap<String, Vec<String>>) {
        match self.op {
            Operation::GetAll => Self::print_employees(department_list),
            Operation::GetDepartment(department) => Self::print_department(department_list, department),
            Operation::Add(name, department) => Self::add_employee(department_list, name, department),
            Operation::Invalid => {
                println!("The input is invalid.")
            },
            _ => (),
        }
    }

    fn get_command(command: Vec<String>) -> Self {
        let first_word = command.get(0);
    
        match first_word {
            Some(word) => {
                match &word[..] {
                    "Get" => Self::get_command_width_get(command),
                    "Add" => Self::get_command_width_add(command),
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

    fn get_command_width_get(command: Vec<String>) -> Self {
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
    fn get_command_width_add(command: Vec<String>) -> Self {
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
        let command = Command::get_command(input);

        if let Operation::Exit = command.op { break; }
        command.execute(&mut departments_list);
    }
}
