use std::fmt::Error;
use std::io::Write;
use chrono::{DateTime, Local};
use serde::__private::de::Content::String;

enum Priority {
    Low,
    Medium,
    High
}


impl Priority{
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Medium => "Medium".to_owned(),
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
//створення таска
    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ").unwrap();

        let description = ConsoleManager::input("Enter new task description: ").unwrap();


        let priority = match ConsoleManager::input("Enter new task priority: ")
            .unwrap()
            .to_losercase()
            .as_str() {
            "low" => Priority::Low,
            "medium" = Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("Invalid priority, setting to low");
                Priority::Low
            }
        };

        Self::new(name, description, priority)
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
//методи роботи з таск (виведення, додавання, видалення, знайти, змінити)
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
struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String()>
}

impl ConsoleManager {

    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(), menu_options: vec![
            "Add task".to_owned(),
            "Find task".to_owned(),
            "Edit task".to_owned(),
            "Remove task".to_owned(),
            "Print task".to_owned(),
            "Store tasks to file".to_owned(),
            "Read tasks from file".to_owned()
        ]
        }
    }

    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }
//реалізація усіх методів
    fn process_command(&mut self) {
        match Self::input("Enter command index: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {
                        self.task_manager.add_task(Task::new_from_console());
                    }
                    "2" => {
                        self.tasks_manager.find_task(match Self::input("Enter task name to find: ") {
                            Ok(name) => name.as_str(),
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        });
                    }
                    "3" => {
                        let name = (match Self::input("Enter task name to edit: ") {
                            Ok(name) => name.as_str(),
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.edit_task(name) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => {}
                        }
                    }
                    "4" => {
                        self.tasks_manager.remove_task()
                    }
                    "5" => {
                        self.tasks_manager.print_tasks()
                    }
                    "6" => {

                    }
                    "7" => {

                    }

                    _ => println!("Sho ty kurwa napisal, close programu chort")
                }
            }
            Err(err) => println!("Error getting user input: {err}")
        }
    }
}

fn main() {

}
