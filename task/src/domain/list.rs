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
