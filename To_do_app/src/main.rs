use std::io;


#[derive(Debug, Clone)]
enum Priority{
    High,
    Medium,
    Low
}

#[derive(Debug)]
struct Task {
    priority: Priority,
    to_do: String,

}

impl Task {
    fn new(priority: Priority, to_do: String) -> Task {
        Task { priority, to_do}
    }
}

#[derive(Debug)]
struct ToDOList {
    high_priority: Vec<Task>,
    medium_priority: Vec<Task>,
    low_priority: Vec<Task>,

}

impl ToDOList {

    fn new(priority: Priority, to_do: String) -> ToDOList {

        // Task created once cannot be modified
        let task = Task::new(priority.clone(), to_do);

        let mut high: Vec<Task> = Vec::new();
        let mut medium: Vec<Task> = Vec::new();
        let mut low: Vec<Task> = Vec::new();

        match priority {
            Priority::High => high.push(task),
            Priority::Medium => medium.push(task),
            Priority::Low => low.push(task),
        }
        
        ToDOList { high_priority: high, medium_priority: medium, low_priority: low }
    }

    fn add_task(&mut self, priority: Priority, to_do: String) {
        
        let task = Task::new(priority.clone(), to_do);

        match priority {
            Priority::High => self.high_priority.push(task),
            Priority::Medium => self.medium_priority.push(task),
            Priority::Low => self.low_priority.push(task),
        }
    }


    fn remove_task(&mut self, priority: Priority, index: usize) {

        match priority {
            Priority::High => self.high_priority.remove(index),
            Priority::Medium => self.medium_priority.remove(index),
            Priority::Low => self.low_priority.remove(index),
        };

    }

    fn print_tasks(&self) {
        let mut count: usize = 1;
        println!("HIGH PRIORITY TASKS!!!!");
        for element in &self.high_priority {
            println!("{})  {}", count, &element.to_do);
            count += 1;
        }
        count = 1;
        println!();
        println!("MEDIUM PRIORITY TASKS!!!!");
        for element in &self.medium_priority {
            println!("{})  {}", count, &element.to_do);
            count += 1;
        }
        count = 1;
        println!();
        println!("LOW PRIORITY TASKS!!!!");
        for element in &self.low_priority {
            println!("{})  {}", count, &element.to_do);
            count += 1;
        }
    }


}

fn main() {

    println!("TO-DO-LIST");
    println!("Create your first Task");
    let mut priority = String::new();
    println!("Task priority(high as h, medium as m, low as l)");
    io::stdin().read_line(&mut priority)
        .expect("Error while reading the message!!");

    let priority = priority.trim();

    let mut task:String = String::new();
    println!("Enter your task: ");
    io::stdin().read_line(&mut task)
        .expect("Error while reading the line");

    let task = task.trim();

    let mut to_do_list = match priority {
        "h" => ToDOList::new(Priority::High, task.to_string()),
        "m" => ToDOList::new(Priority::Medium, task.to_string()),
        "l" => ToDOList::new(Priority::Low, task.to_string()),
        _ => {
            println!("Invalid type!!");
            return;
        }
    };

    println!("Todo list created successfully!!!!");


    loop {
        println!("Todo list option:");
        println!("1) Display todo list");
        println!("2) Add task");
        println!("3) Remove task");
        println!("4) Exit");
        println!("Choose option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Error while reading the line");

        let choice: u8 = choice.trim().parse().unwrap();

        match choice {
            1 => to_do_list.print_tasks(),
            2 => {
                println!("Task priority(high as h, medium as m, low as l)");
                let mut priority = String::new();
                io::stdin().read_line(&mut priority)
                    .expect("Error while reading the message!!");

                let priority = priority.trim();

                let mut task: String = String::new();
                println!("Enter your task: ");
                io::stdin().read_line(&mut task)
                    .expect("Error while reading the line");

                let task = task.trim();
                match priority {
                    "h" => to_do_list.add_task(Priority::High, task.to_string()),
                    "m" => to_do_list.add_task(Priority::Medium, task.to_string()),
                    "l" => to_do_list.add_task(Priority::Low, task.to_string()),
                    _ => {
                            println!("Invalid type!!");
                            println!("failed to add the task!!!");
                            continue;
                        }
                }
            }
            3 => {
                to_do_list.print_tasks();
                let mut priority = String::new();
                println!("priority(high as h, medium as m, low as l)");
                io::stdin().read_line(&mut priority)
                    .expect("Error while reading the line");

                let priority: &str  = priority.trim();

                let mut index = String::new();
                println!("Enter the tasks index to delete");
                io::stdin().read_line(&mut index)
                    .expect("Error while reading the line");

                let index: usize = index.trim().parse().unwrap();

                match priority {
                    "h" => to_do_list.remove_task(Priority::High, index-1 ),
                    "m" => to_do_list.remove_task(Priority::Medium, index-1),
                    "l" => to_do_list.remove_task(Priority::Low, index-1),
                    _ => {
                        println!("Invalid priority");
                        continue;
                    }
                }

            }
            4 => {
                println!("All the tasks will be deleted!!!");
                println!("Thank you for using my to-do-app");
                break;
            }
            _ => {
                println!("Invalid choice");
                continue;
            }
        }
        println!();
        
    }

}