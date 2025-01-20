use shared_kernel::Id;
use thiserror::Error;

use super::{
    net::{Net, Status},
    task::Task,
};

#[derive(Debug, Error)]
pub enum TaskDomainError {
    #[error("status {status:?} not found in net {net:?}")]
    StatusNotFoundInNet { net: Id<Net>, status: Id<Status> },

    #[error("task {task:?} not found in net {net:?}")]
    TaskNotFoundInNet { net: Id<Net>, task: Id<Task> },

    #[error("relation from {from:?} to {to:?} not found in net {net:?}")]
    RelationNotFoundInNet {
        net: Id<Net>,
        from: Id<Task>,
        to: Id<Task>,
    },

    #[error("relation constraints not satisfied for task {task:?} in net {net:?}")]
    RelationConstraintNotSatisfied { net: Id<Net>, task: Id<Task> },

    #[error("task {task:?} already in net {net:?}")]
    TaskAlreadyInNet { task: Id<Task>, net: Id<Net> },

    #[error("status {status:?} is default status in net {net:?}")]
    StatusNotRemovable { net: Id<Net>, status: Id<Status> },

    #[error("cycle found in net {0:?}")]
    CycleNotAllowedInNet(Id<Net>),
}
