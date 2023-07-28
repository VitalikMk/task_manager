use std::fmt::format;
use chrono::{DateTime, Local};

enum Priority {
    Low,
    Medium,
    High
}


impl Priority{
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>
}


impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self { name, description, priority, add_time: Local::now() }
    }


    fn print_task(&self) {
        println!("{} | {} | {}\n\"{}\"",
                 self.name,
                 self.priority.to_string(),
                 self.add_time.format("%Y-%m-%d %H:%M:%S"),
                 self.description
        )
    }
}

struct TasksManager {
    tasks: Vec<Task>
}
//методи роботи з таск (виведення, додавання, видалення, знайти)
impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed good", name))
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }

    }



    fn edit_task(&mut self, name: String, updated_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(&name) {
            match self.tasks.get_mut(index) {
                None => Err("Error borrowing task".to_owned()),
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;
                    Ok(format!("Task \"{}\" removed successfully", name))
                }
            }

        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

}

fn main() {

}
