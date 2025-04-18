
#[derive(Clone)]
    pub enum Priority {
        High,
        Medium,
        Low
    }
    
    #[derive(Debug)]
    pub struct ToDoList {
        high_priority: Vec<String>,
        medium_priority: Vec<String>,
        low_priority: Vec<String>,
        
    }
    
impl ToDoList {
    
    pub fn new() -> ToDoList {
        
        let high_priority=  Vec::new();
        let medium_priority=  Vec::new();
        let low_priority=  Vec::new();
        
        ToDoList { high_priority, medium_priority, low_priority }
    }
    
    pub fn add_task(&mut self, task: String, priority: Priority) {
        match priority {
            Priority::High => self.high_priority.push(task),
            Priority::Medium => self.medium_priority.push(task),
            Priority::Low => self.low_priority.push(task),
        }
    }
    
    pub fn remove_task(&mut self, priority: Priority, index: usize) -> String{


        let lenght: usize;
        
        match priority {
            Priority::High => {lenght = self.high_priority.len()},
            Priority::Medium => {lenght = self.medium_priority.len()},
            Priority::Low => {lenght = self.low_priority.len()}, 
        };
        
        if index - 1 > lenght {
            panic!("Invalid Index");
        }
        
        //removing the task
        match priority {
            Priority::High => self.high_priority.remove(index - 1),
            Priority::Medium => self.medium_priority.remove(index - 1),
            Priority::Low => self.low_priority.remove(index - 1), 
        }
    }

    pub fn get_len(&self, priority: Priority) -> usize {
        match priority {
            Priority::High => self.high_priority.len(),
            Priority::Medium => self.medium_priority.len(),
            Priority::Low => self.low_priority.len(),
        }
    }

    pub fn show(&self) {
        println!("\n");
        let mut count: usize = 1;
        println!("HIGH PRIORITY TASKS!!!!");
        for element in &self.high_priority {
            println!("{})  {}", count, &element);
            count += 1;
        }

        count = 1;
        println!();
        println!("MEDIUM PRIORITY TASKS!!!!");
        for element in &self.medium_priority {
            println!("{})  {}", count, &element);
            count += 1;
        }

        count = 1;
        println!();
        println!("LOW PRIORITY TASKS!!!!");
        for element in &self.low_priority {
            println!("{})  {}", count, &element);
            count += 1;
        }
    

        


    }
            
}


