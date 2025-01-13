use shared_kernel::{Entity, Id};

use super::list::List;

pub struct Task {
    pub name: String,
    pub list: Id<List>,
}

pub trait TaskAggregateRoot {
    fn rename(&mut self, name: String);
}

impl TaskAggregateRoot for Entity<Task> {
    fn rename(&mut self, name: String) {
        self.data.name = name;
    }
}
