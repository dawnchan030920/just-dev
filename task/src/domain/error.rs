use shared_kernel::Id;
use thiserror::Error;

use super::{
    net::{Net, Status},
    task::Task,
};

/// Represents errors that can occur in the task domain.
#[derive(Debug, Error)]
pub enum TaskDomainError {
    /// Error indicating that a status was not found in a net.
    #[error("status {status:?} not found in net {net:?}")]
    StatusNotFoundInNet { net: Id<Net>, status: Id<Status> },

    /// Error indicating that a task was not found in a net.
    #[error("task {task:?} not found in net {net:?}")]
    TaskNotFoundInNet { net: Id<Net>, task: Id<Task> },

    /// Error indicating that a relation was not found in a net.
    #[error("relation from {from:?} to {to:?} not found in net {net:?}")]
    RelationNotFoundInNet {
        net: Id<Net>,
        from: Id<Task>,
        to: Id<Task>,
    },

    /// Error indicating that relation constraints were not satisfied for a task in a net.
    #[error("relation constraints not satisfied for task {task:?} in net {net:?}")]
    RelationConstraintNotSatisfied { net: Id<Net>, task: Id<Task> },

    /// Error indicating that a task is already in a net.
    #[error("task {task:?} already in net {net:?}")]
    TaskAlreadyInNet { task: Id<Task>, net: Id<Net> },

    /// Error indicating that a status is the default status in a net and cannot be removed.
    #[error("status {status:?} is default status in net {net:?}")]
    StatusNotRemovable { net: Id<Net>, status: Id<Status> },

    /// Error indicating that a cycle was found in a net, which is not allowed.
    #[error("cycle found in net {0:?}")]
    CycleNotAllowedInNet(Id<Net>),
}
