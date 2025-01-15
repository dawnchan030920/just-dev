use std::collections::HashMap;

use shared_kernel::{Entity, Id};

use super::{error::TaskDomainError, task::Task};

#[derive(Debug)]
pub struct Net {
    relations: Vec<Relation>,
    schema: Schema,
    tasks: HashMap<Id<Task>, Id<Status>>,
}

#[derive(Debug)]
pub struct Relation {
    from: Id<Task>,
    to: Id<Task>,
    relation_type: RelationType,
}

#[derive(Debug)]
pub enum RelationType {
    Compose,
    Require,
}

#[derive(Debug)]
pub struct Schema {
    status: Vec<Entity<Status>>,
    default: Id<Status>,
    accepted: Id<Status>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Status {
    name: String,
}

type TaskDomainResult = Result<(), TaskDomainError>;

pub trait NetAggregateRoot {
    fn change_status_name(&mut self, status_id: Id<Status>, new_name: String) -> TaskDomainResult;
    fn change_default(&mut self, new_default: Id<Status>) -> TaskDomainResult;
    fn add_task(&mut self, task_id: Id<Task>) -> TaskDomainResult;
    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult;
    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult;
    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult;
    fn change_task_status(&mut self, task_id: Id<Task>, status_id: Id<Status>) -> TaskDomainResult;
}

impl NetAggregateRoot for Entity<Net> {
    fn change_status_name(&mut self, status_id: Id<Status>, new_name: String) -> TaskDomainResult {
        todo!()
    }

    fn change_default(&mut self, new_default: Id<Status>) -> TaskDomainResult {
        todo!()
    }

    fn add_task(&mut self, task_id: Id<Task>) -> TaskDomainResult {
        todo!()
    }

    fn change_task_status(&mut self, task_id: Id<Task>, status_id: Id<Status>) -> TaskDomainResult {
        todo!()
    }

    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult {
        todo!()
    }

    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult {
        todo!()
    }

    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult {
        todo!()
    }
}
