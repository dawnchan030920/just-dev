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

    #[error("relation not found in net {net:?}")]
    RelationNotFoundInNet { net: Id<Net> },

    #[error("relation boundary on task {task:?} not satisfied:\n\tRequirements: {requirements:?}\n\tSubtasks: {subtasks:?}")]
    RelationBoundaryNotSatisfied {
        task: Id<Task>,
        requirements: Vec<Id<Task>>,
        subtasks: Vec<Id<Task>>,
    },

    #[error("task {task:?} already in net {net:?}")]
    TaskAlreadyInNet { task: Id<Task>, net: Id<Net> },

    #[error("status {status:?} is default status in net {net:?}")]
    StatusNotRemovable { net: Id<Net>, status: Id<Status> },
}
