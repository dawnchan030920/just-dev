use std::collections::HashMap;

use shared_kernel::{Entity, Id};

use super::task::Task;

pub struct Net {
    relations: Vec<Relation>,
    schema: Schema,
    tasks: HashMap<Id<Task>, Id<Status>>,
}

pub struct Relation {
    from: Id<Task>,
    to: Id<Task>,
    relation_type: RelationType,
}

pub enum RelationType {
    Compose,
    Require,
}

pub struct Schema {
    status: Vec<Entity<Status>>,
    default: Id<Status>,
    accepted: Id<Status>,
}

pub struct Status {
    name: String,
}

pub trait NetAggregateRoot {}

impl NetAggregateRoot for Entity<Net> {}
