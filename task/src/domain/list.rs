use shared_kernel::{Entity, Id};

#[derive(Debug)]
pub struct List {
    title: String,
}

pub trait ListAggregateRoot {
    fn rename(&mut self, title: String);
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
