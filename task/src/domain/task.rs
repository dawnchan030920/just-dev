use shared_kernel::{Entity, Id};

use super::list::List;

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub list: Id<List>,
}

pub trait TaskAggregateRoot {
    fn rename(&mut self, name: String);
    fn new(name: String, list: Id<List>) -> Self;
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
