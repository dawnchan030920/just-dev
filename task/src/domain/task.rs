use shared_kernel::{Entity, Id};

use super::list::List;

/// Represents a task.
#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub list: Id<List>,
}

/// Trait for aggregate root operations on a `Task`.
pub trait TaskAggregateRoot {
    /// Renames the task with a new name.
    fn rename(&mut self, name: String);

    /// Creates a new task with the given name and list.
    fn new(name: String, list: Id<List>) -> Self;

    /// Categorizes the task to a new list.
    fn categorize_to(&mut self, list: Id<List>);
}

impl TaskAggregateRoot for Entity<Task> {
    fn rename(&mut self, name: String) {
        self.data.name = name;
    }

    fn new(name: String, list: Id<List>) -> Self {
        Entity {
            id: Id::new(),
            data: Task { name, list },
        }
    }

    fn categorize_to(&mut self, list: Id<List>) {
        self.data.list = list;
    }
}
