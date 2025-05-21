use std::collections::HashMap;
use crate::task::Task;

pub struct TaskStorage {
    tasks: HashMap<u32, Task>
}

impl TaskStorage {
    pub fn new() -> Self {
       Self {tasks: HashMap::new()}
    }

    pub fn add(&mut self, new_task: Task) {
        self.tasks.insert(new_task.id, new_task);
    }

    pub fn view_all(&self) {
        if self.tasks.is_empty() {
            println!("No tasks!");
        } else {
            for task in self.tasks.values() {
                task.print();
            }
        }
    }

    pub fn mark_completed(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.mark_completed();
                Ok(())
            },
            None => Err("Task is not found".to_string())
        }
    }

    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.remove(&id) {
            Some(_) => Ok(()),
            None => Err("Task is not found".to_string())
        }
    }
}

