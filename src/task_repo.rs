use crate::task::Task;

pub trait TaskRepo {
    /// Add a task
    fn add(&mut self, task: Task);

    /// List all tasks 
    fn list(&self) -> Vec<Task>;
}