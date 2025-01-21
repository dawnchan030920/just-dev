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

#[cfg(test)]
mod tests {
    use super::*;
    use shared_kernel::Id;

    #[test]
    fn test_status_not_found_in_net() {
        let net_id = Id::new();
        let status_id = Id::new();
        let error = TaskDomainError::StatusNotFoundInNet {
            net: net_id,
            status: status_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("status {:?} not found in net {:?}", status_id, net_id)
        );
    }

    #[test]
    fn test_task_not_found_in_net() {
        let net_id = Id::new();
        let task_id = Id::new();
        let error = TaskDomainError::TaskNotFoundInNet {
            net: net_id,
            task: task_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("task {:?} not found in net {:?}", task_id, net_id)
        );
    }

    #[test]
    fn test_relation_not_found_in_net() {
        let net_id = Id::new();
        let from_id = Id::new();
        let to_id = Id::new();
        let error = TaskDomainError::RelationNotFoundInNet {
            net: net_id,
            from: from_id,
            to: to_id,
        };
        assert_eq!(
            format!("{}", error),
            format!(
                "relation from {:?} to {:?} not found in net {:?}",
                from_id, to_id, net_id
            )
        );
    }

    #[test]
    fn test_relation_constraint_not_satisfied() {
        let net_id = Id::new();
        let task_id = Id::new();
        let error = TaskDomainError::RelationConstraintNotSatisfied {
            net: net_id,
            task: task_id,
        };
        assert_eq!(
            format!("{}", error),
            format!(
                "relation constraints not satisfied for task {:?} in net {:?}",
                task_id, net_id
            )
        );
    }

    #[test]
    fn test_task_already_in_net() {
        let net_id = Id::new();
        let task_id = Id::new();
        let error = TaskDomainError::TaskAlreadyInNet {
            task: task_id,
            net: net_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("task {:?} already in net {:?}", task_id, net_id)
        );
    }

    #[test]
    fn test_status_not_removable() {
        let net_id = Id::new();
        let status_id = Id::new();
        let error = TaskDomainError::StatusNotRemovable {
            net: net_id,
            status: status_id,
        };
        assert_eq!(
            format!("{}", error),
            format!(
                "status {:?} is default status in net {:?}",
                status_id, net_id
            )
        );
    }

    #[test]
    fn test_cycle_not_allowed_in_net() {
        let net_id = Id::new();
        let error = TaskDomainError::CycleNotAllowedInNet(net_id);
        assert_eq!(
            format!("{}", error),
            format!("cycle found in net {:?}", net_id)
        );
    }
}
