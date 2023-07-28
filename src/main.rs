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
        println!("{} | {} | {}",
                 self.name,
                 self.priority.to_string(),
                 self.add_time.format("%Y-%m-%d %H:%M:%S")
        )
    }
}


fn main() {


}
