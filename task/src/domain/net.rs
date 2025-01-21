use std::collections::HashMap;

use petgraph::{
    algo::{has_path_connecting, toposort},
    prelude::DiGraphMap,
    Direction::Incoming,
};
use shared_kernel::{Entity, Id};

use super::{error::TaskDomainError, task::Task};

/// Represents a network of tasks and their relations.
#[derive(Debug)]
pub struct Net {
    relations: DiGraphMap<Id<Task>, RelationType>,
    schema: Schema,
    tasks: HashMap<Id<Task>, Id<Status>>,
}

/// Represents the type of relation between tasks.
#[derive(Debug, PartialEq, Eq)]
pub enum RelationType {
    /// A composition relation.
    Compose,
    /// A requirement relation.
    Require,
}

/// Represents the status schema of a network, including statuses and default/accepted statuses.
#[derive(Debug)]
pub struct Schema {
    status: Vec<Entity<Status>>,
    default: Id<Status>,
    accepted: Id<Status>,
}

/// Represents the status of a task.
#[derive(Debug, PartialEq, Eq)]
pub struct Status {
    name: String,
}

type TaskDomainResult<T> = Result<T, TaskDomainError>;

/// Trait for aggregate root operations on a `Net`.
pub trait NetAggregateRoot {
    /// Adds a new status to the network.
    fn new_status(&mut self, status_name: String);
    /// Removes a status from the network.
    fn remove_status(&mut self, status_id: Id<Status>) -> TaskDomainResult<()>;
    /// Changes the name of a status in the network.
    fn change_status_name(
        &mut self,
        status_id: Id<Status>,
        new_name: String,
    ) -> TaskDomainResult<()>;
    /// Changes the default status of the network.
    fn change_default(&mut self, new_default: Id<Status>) -> TaskDomainResult<()>;
    /// Adds a new task to the network.
    fn add_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()>;
    /// Removes a task from the network.
    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()>;
    /// Adds a new relation between tasks in the network.
    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult<()>;
    /// Removes a relation between tasks in the network.
    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult<()>;
    /// Changes the status of a task in the network.
    fn change_task_status(
        &mut self,
        task_id: Id<Task>,
        status_id: Id<Status>,
    ) -> TaskDomainResult<()>;
}

/// Propagates changes through all tasks in the network.
fn propagate_all(net: &mut Entity<Net>) -> TaskDomainResult<()> {
    propagate(net, |tasks| tasks)
}

/// Propagates changes from a specific task in the network.
fn propagate_from(net: &mut Entity<Net>, task: &Id<Task>) -> TaskDomainResult<()> {
    propagate(net, |tasks| {
        tasks
            .into_iter()
            .skip_while(|t| *t != *task)
            .skip(1)
            .collect()
    })
}

/// Propagates changes at a specific task in the network.
fn propagate_at(net: &mut Entity<Net>, task: &Id<Task>) -> TaskDomainResult<()> {
    propagate(net, |tasks| {
        tasks.into_iter().skip_while(|t| *t != *task).collect()
    })
}

/// Propagates changes through tasks in the network using a transformation function applied to toposorted task list.
fn propagate<F>(net: &mut Entity<Net>, sorted_tasks_transform: F) -> TaskDomainResult<()>
where
    F: Fn(Vec<Id<Task>>) -> Vec<Id<Task>>,
{
    let tasks: Vec<_> = toposort(&net.data.relations, None)
        .map_err(|_| TaskDomainError::CycleNotAllowedInNet(net.id))?;

    let tasks = sorted_tasks_transform(tasks);

    for task in tasks {
        if let Some(accepted) = is_controlled_task_accepted(net, &task)? {
            let stored_task_status = net
                .data
                .tasks
                .get_mut(&task)
                .ok_or(TaskDomainError::TaskNotFoundInNet { net: net.id, task })?;

            let stored_task_accepted = *stored_task_status == net.data.schema.accepted;

            if accepted != stored_task_accepted {
                match accepted {
                    true => *stored_task_status = net.data.schema.accepted,
                    false => *stored_task_status = net.data.schema.default,
                }
            }
        }
    }

    Ok(())
}

/// Checks if a controlled task is accepted in the network.
fn is_controlled_task_accepted(
    net: &Entity<Net>,
    task: &Id<Task>,
) -> TaskDomainResult<Option<bool>> {
    let incoming_edges = net
        .data
        .relations
        .edges_directed(*task, Incoming)
        .into_iter();

    let mut have_subtasks = false;
    for incoming_edge in incoming_edges {
        let relation_type = incoming_edge.2;
        let task_id = incoming_edge.0;

        if *net
            .data
            .tasks
            .get(&task_id)
            .ok_or(TaskDomainError::TaskNotFoundInNet {
                net: net.id,
                task: task_id,
            })?
            != net.data.schema.accepted
        {
            return Ok(Some(false));
        }

        if *relation_type == RelationType::Compose {
            have_subtasks = true;
        }
    }
    if have_subtasks {
        return Ok(Some(true));
    }
    Ok(None)
}

impl NetAggregateRoot for Entity<Net> {
    fn change_status_name(
        &mut self,
        status_id: Id<Status>,
        new_name: String,
    ) -> TaskDomainResult<()> {
        self.data
            .schema
            .status
            .iter_mut()
            .find(|status| status.id == status_id)
            .map(|status| {
                status.data.name = new_name;
                ()
            })
            .ok_or(TaskDomainError::StatusNotFoundInNet {
                net: self.id,
                status: status_id,
            })
    }

    fn change_default(&mut self, new_default: Id<Status>) -> TaskDomainResult<()> {
        if !self
            .data
            .schema
            .status
            .iter()
            .any(|status| status.id == new_default)
        {
            return Err(TaskDomainError::StatusNotFoundInNet {
                net: self.id,
                status: new_default,
            });
        }

        for (_, status) in self.data.tasks.iter_mut() {
            if *status == self.data.schema.default {
                *status = new_default;
            }
        }

        self.data.schema.default = new_default;

        Ok(())
    }

    fn add_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()> {
        if self.data.tasks.contains_key(&task_id) {
            return Err(TaskDomainError::TaskAlreadyInNet {
                task: task_id,
                net: self.id,
            });
        }

        self.data.tasks.insert(task_id, self.data.schema.default);
        self.data.relations.add_node(task_id);

        Ok(())
    }

    fn change_task_status(
        &mut self,
        task_id: Id<Task>,
        status_id: Id<Status>,
    ) -> TaskDomainResult<()> {
        if is_controlled_task_accepted(&self, &task_id)?.is_none() {
            let task_status =
                self.data
                    .tasks
                    .get_mut(&task_id)
                    .ok_or(TaskDomainError::TaskNotFoundInNet {
                        net: self.id,
                        task: task_id,
                    })?;
            *task_status = status_id;
            propagate_from(self, &task_id)?;

            Ok(())
        } else {
            return Err(TaskDomainError::RelationConstraintNotSatisfied {
                net: self.id,
                task: task_id,
            });
        }
    }

    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult<()> {
        if has_path_connecting(&self.data.relations, to, from, None) {
            return Err(TaskDomainError::CycleNotAllowedInNet(self.id));
        }

        self.data.relations.add_edge(from, to, relation_type);

        propagate_at(self, &to)?;

        Ok(())
    }

    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()> {
        self.data.tasks.remove(&task_id);
        self.data.relations.remove_node(task_id);

        propagate_all(self)?;

        Ok(())
    }

    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult<()> {
        self.data.relations.remove_edge(from, to);

        propagate_at(self, &to)?;

        Ok(())
    }

    fn new_status(&mut self, status_name: String) {
        self.data.schema.status.push(Entity {
            id: Id::new(),
            data: Status { name: status_name },
        });
    }

    fn remove_status(&mut self, removed_status: Id<Status>) -> TaskDomainResult<()> {
        if !self
            .data
            .schema
            .status
            .iter()
            .any(|status| status.id == removed_status)
        {
            return Err(TaskDomainError::StatusNotFoundInNet {
                net: self.id,
                status: removed_status,
            });
        }

        if removed_status == self.data.schema.default || removed_status == self.data.schema.accepted
        {
            return Err(TaskDomainError::StatusNotRemovable {
                net: self.id,
                status: removed_status,
            });
        }

        for (_, status) in self.data.tasks.iter_mut() {
            if *status == removed_status {
                *status = self.data.schema.default;
            }
        }

        self.data
            .schema
            .status
            .retain(|status| status.id != removed_status);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared_kernel::Id;

    #[test]
    fn test_new_status() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        net.new_status("New Status".to_string());
        assert_eq!(net.data.schema.status.len(), 1);
        assert_eq!(net.data.schema.status[0].data.name, "New Status");
    }

    #[test]
    fn test_remove_status() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let status_id = Id::new();
        net.data.schema.status.push(Entity {
            id: status_id,
            data: Status {
                name: "Status to Remove".to_string(),
            },
        });

        net.remove_status(status_id).unwrap();
        assert!(net.data.schema.status.is_empty());
    }

    #[test]
    fn test_change_status_name() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let status_id = Id::new();
        net.data.schema.status.push(Entity {
            id: status_id,
            data: Status {
                name: "Old Status Name".to_string(),
            },
        });

        net.change_status_name(status_id, "New Status Name".to_string())
            .unwrap();
        assert_eq!(net.data.schema.status[0].data.name, "New Status Name");
    }

    #[test]
    fn test_change_default() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let status_id = Id::new();
        net.data.schema.status.push(Entity {
            id: status_id,
            data: Status {
                name: "Status".to_string(),
            },
        });

        net.change_default(status_id).unwrap();
        assert_eq!(net.data.schema.default, status_id);
    }

    #[test]
    fn test_add_task() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let task_id = Id::new();
        net.add_task(task_id).unwrap();
        assert_eq!(net.data.tasks.len(), 1);
        assert_eq!(net.data.tasks[&task_id], net.data.schema.default);
    }

    #[test]
    fn test_remove_task() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let task_id = Id::new();
        net.add_task(task_id).unwrap();
        net.remove_task(task_id).unwrap();
        assert!(net.data.tasks.is_empty());
    }

    #[test]
    fn test_new_relation() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let task_id_1 = Id::new();
        let task_id_2 = Id::new();
        net.add_task(task_id_1).unwrap();
        net.add_task(task_id_2).unwrap();

        net.new_relation(task_id_1, task_id_2, RelationType::Compose)
            .unwrap();
        assert!(net.data.relations.contains_edge(task_id_1, task_id_2));
    }

    #[test]
    fn test_remove_relation() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let task_id_1 = Id::new();
        let task_id_2 = Id::new();
        net.add_task(task_id_1).unwrap();
        net.add_task(task_id_2).unwrap();

        net.new_relation(task_id_1, task_id_2, RelationType::Compose)
            .unwrap();
        net.remove_relation(task_id_1, task_id_2).unwrap();
        assert!(!net.data.relations.contains_edge(task_id_1, task_id_2));
    }

    #[test]
    fn test_change_task_status() {
        let mut net = Entity::new(Net {
            relations: DiGraphMap::new(),
            schema: Schema {
                status: vec![],
                default: Id::new(),
                accepted: Id::new(),
            },
            tasks: HashMap::new(),
        });

        let task_id = Id::new();
        net.add_task(task_id).unwrap();

        let status_id = Id::new();
        net.data.schema.status.push(Entity {
            id: status_id,
            data: Status {
                name: "Status".to_string(),
            },
        });

        net.change_task_status(task_id, status_id).unwrap();
        assert_eq!(net.data.tasks[&task_id], status_id);
    }
}
