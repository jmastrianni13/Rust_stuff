use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

pub fn emp_app() {
    let mut dept_emp_map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
         
        let parsed_command = parse_command(command);
        match parsed_command.cmp(&String::from("exit")) {
            Ordering::Equal => {
                println!("bye");
                break;
            },
            _ => {
               handle_command(parsed_command, &mut dept_emp_map);
            }
        }
    }
}

fn handle_command(command: String, dept_emp_map: &mut HashMap<String, Vec<String>>) {
    let command_parts = get_command_parts(command.as_str());
    let first_part = &command_parts[0];
    
    match first_part {
        &"company" => print_company(dept_emp_map),
        &"Add" => {
            let command_emp = get_command_emp(&command_parts);
            let command_dept = get_command_dept(&command_parts);
            add_employee(command_emp, command_dept, dept_emp_map);
        },
        _ => print_employees_by_dept(first_part, dept_emp_map),
    }
}

fn add_employee(name: String, dept: String, dept_emp_map: &mut HashMap<String, Vec<String>>) {
    dept_emp_map.entry(dept).or_insert_with(|| Vec::new()).push(name);
}

fn parse_command(command: String) -> String {
     let command: String = match command.trim().parse() {
         Ok(command_string) => command_string,
         Err(_) => todo!(),
     };
     return command;
}

fn get_command_parts(command: &str) -> Vec<&str> {
    let command_parts = command.split(" ").collect::<Vec<&str>>();
    return command_parts;
}

fn get_command_emp(command_parts: &[&str]) -> String {
    return command_parts[1].to_string();
}

fn get_command_dept(command_parts: &[&str]) -> String {
    return command_parts[3].to_string();
}

 
fn print_employees_by_dept(dept: &str, dept_emp_map: &mut HashMap<String, Vec<String>>) {
    for emp in dept_emp_map.get(dept).unwrap() {
        println!("  {emp}");
    }
}

fn print_company(dept_emp_map: &mut HashMap<String, Vec<String>>) {
    for (dept, emps) in dept_emp_map {
        println!("{dept}:");
        for emp in emps {
            println!("  {emp}");
        }
    }
}
