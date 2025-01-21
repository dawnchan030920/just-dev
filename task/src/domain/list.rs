use shared_kernel::{Entity, Id};

/// Represents a task list.
#[derive(Debug)]
pub struct List {
    title: String,
}

/// Trait for aggregate root operations on a `List`.
pub trait ListAggregateRoot {
    /// Renames the list with a new title.
    fn rename(&mut self, title: String);

    /// Creates a new list with the given title.
    fn new(title: String) -> Self;
}

impl ListAggregateRoot for Entity<List> {
    fn rename(&mut self, title: String) {
        self.data.title = title;
    }

    fn new(title: String) -> Self {
        Entity {
            id: Id::new(),
            data: List { title },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename() {
        let mut list = Entity::new("Test List".to_string());
        list.rename("Renamed List".to_string());
        assert_eq!(list.data.title, "Renamed List");
    }

    #[test]
    fn test_new() {
        let list = Entity::new("New List".to_string());
        assert_eq!(list.data.title, "New List");
    }
}
