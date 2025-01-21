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

#[cfg(test)]
mod tests {
    use super::*;
    use shared_kernel::Id;
    use super::super::list::List;

    #[test]
    fn test_rename() {
        let list_id = Id::new();
        let mut task = Entity::new("Test Task".to_string(), list_id);
        task.rename("Renamed Task".to_string());
        assert_eq!(task.data.name, "Renamed Task");
    }

    #[test]
    fn test_new() {
        let list_id = Id::new();
        let task = Entity::new("New Task".to_string(), list_id);
        assert_eq!(task.data.name, "New Task");
        assert_eq!(task.data.list, list_id);
    }

    #[test]
    fn test_categorize_to() {
        let list_id = Id::new();
        let new_list_id = Id::new();
        let mut task = Entity::new("Test Task".to_string(), list_id);
        task.categorize_to(new_list_id);
        assert_eq!(task.data.list, new_list_id);
    }
}
