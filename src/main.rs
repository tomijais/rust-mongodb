mod task;
mod in_memory_task_repo;
mod task_repo;

use task::Task;
use task_repo::TaskRepo;

use crate::in_memory_task_repo::InMemoryTaskRepo;
fn main() {
    let task = Task::new("Buy milk".to_string(), false, None);

    println!("Task: {task:?}");

    let mut repo: InMemoryTaskRepo = InMemoryTaskRepo::new();
    repo.add(task);
    println!("Repo: {repo:?}");
}
