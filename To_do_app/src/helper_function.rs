use std::{io, usize};
use crate::to_do_list::{Priority, ToDoList};
#[allow(non_snake_case)]
pub fn Add_task(to_do_list: &mut ToDoList){

    let mut task: String;

    loop {

        println!("Enter Task:");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_string) => {
                task = input.trim().to_string();
                if task == "".to_string() {
                        println!("No Task was Given!!!");
                        continue;
                }
                break;
            },
            Err(e) => {
                println!("{}", e);
            }
        }  
    }
    
    let priority: Priority;

    loop {
        println!("Enter the priority (High: H, Medium: M, Low: L): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_string) => {
                let input = input.trim();
                match input {
                    "H" => {
                        priority = Priority::High;
                        break;
                    },
                    "M" => {
                        priority = Priority::Medium;
                        break;
                    },
                    "L" => {
                        priority = Priority::Low;
                        break;
                    },
                    _ => {
                        println!("Invalid priority!!!");
                        continue;
                    }


                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    to_do_list.add_task(task, priority);
    println!("Successfully added the task!!!");    
}

#[allow(non_snake_case)]
pub fn Remove_task(to_do_list: &mut ToDoList) {

    let priority: Priority;

    loop {
        println!("Enter the priority (High: H, Medium: M, Low: L): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_string) => {
                let input = input.trim();
                match input {
                    "H" => {
                        priority = Priority::High;
                        break;
                    },
                    "M" => {
                        priority = Priority::Medium;
                        break;
                    },
                    "L" => {
                        priority = Priority::Low;
                        break;
                    },
                    _ => {
                        println!("Invalid priority!!!");
                        continue;
                    }


                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }


    let index: usize;

     loop {

        println!("Enter the index that you want to delete: ");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_string) => {
                match input.trim().parse::<usize>() {
                        Ok(num) => {
                            if to_do_list.get_len(priority.clone()) < num -1 {
                                println!("Invalid index");
                                continue;
                            } else {
                                index = num;
                                break;
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }  
    }

    let removed_task = to_do_list.remove_task(priority, index);
    println!("The task '{}' removed successfully", removed_task);
}




