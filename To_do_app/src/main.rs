use std::io::{self};
mod to_do_list;
use to_do_list::*;
mod helper_function;
use helper_function::*;




fn main () {

    let mut to_do_app = ToDoList::new();

    println!("WELCOME TO TO-DO-APP");
    loop {
        println!("\n");
        println!("Todo list option:");
        println!("1) Display todo list");
        println!("2) Add task");
        println!("3) Remove task");
        println!("4) Exit");
        println!("Choose option: ");

        let mut input = String::new();
        let choice: u8;
        match io::stdin().read_line(&mut input) {
            Ok(_string) => {},
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
        
        match input.trim().parse::<u8>() {
            Ok(num) => {
                choice = num;
                
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        match choice {
            1 => to_do_app.show(),
            2 => Add_task(&mut to_do_app),
            3 => Remove_task(&mut to_do_app),
            4 => {
                println!("Deleting all the tasks!!!");
                println!("Thank you for using!!!");
                break;
            }
            _ => continue,
        }

    }

}