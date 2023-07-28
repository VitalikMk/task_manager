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

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_tasks(&self) {
        for task in self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

fn main() {

}
