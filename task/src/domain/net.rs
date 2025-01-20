use std::collections::HashMap;

use petgraph::{
    algo::{has_path_connecting, toposort},
    prelude::DiGraphMap,
    Direction::Incoming,
};
use shared_kernel::{Entity, Id};

use super::{error::TaskDomainError, task::Task};

#[derive(Debug)]
pub struct Net {
    relation_graph: DiGraphMap<Id<Task>, RelationType>,
    schema: Schema,
    tasks: HashMap<Id<Task>, Id<Status>>,
}

#[derive(Debug, PartialEq, Eq)]
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

type TaskDomainResult<T> = Result<T, TaskDomainError>;

pub trait NetAggregateRoot {
    fn new_status(&mut self, status_name: String);
    fn remove_status(&mut self, status_id: Id<Status>) -> TaskDomainResult<()>;
    fn change_status_name(
        &mut self,
        status_id: Id<Status>,
        new_name: String,
    ) -> TaskDomainResult<()>;
    fn change_default(&mut self, new_default: Id<Status>) -> TaskDomainResult<()>;
    fn add_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()>;
    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()>;
    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult<()>;
    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult<()>;
    fn change_task_status(
        &mut self,
        task_id: Id<Task>,
        status_id: Id<Status>,
    ) -> TaskDomainResult<()>;
}

fn propagate_all(net: &mut Entity<Net>) -> TaskDomainResult<()> {
    propagate(net, |tasks| tasks)
}

fn propagate_from(net: &mut Entity<Net>, task: &Id<Task>) -> TaskDomainResult<()> {
    propagate(net, |tasks| {
        tasks
            .into_iter()
            .skip_while(|t| *t != *task)
            .skip(1)
            .collect()
    })
}

fn propagate_at(net: &mut Entity<Net>, task: &Id<Task>) -> TaskDomainResult<()> {
    propagate(net, |tasks| {
        tasks.into_iter().skip_while(|t| *t != *task).collect()
    })
}

fn propagate<F>(net: &mut Entity<Net>, sorted_tasks_transform: F) -> TaskDomainResult<()>
where
    F: Fn(Vec<Id<Task>>) -> Vec<Id<Task>>,
{
    let tasks: Vec<_> = toposort(&net.data.relation_graph, None).unwrap();

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

fn is_controlled_task_accepted(
    net: &Entity<Net>,
    task: &Id<Task>,
) -> TaskDomainResult<Option<bool>> {
    let incoming_edges = net
        .data
        .relation_graph
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
        self.data.relation_graph.add_node(task_id);

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
        if has_path_connecting(&self.data.relation_graph, to, from, None) {
            return Err(TaskDomainError::CycleNotAllowedInNet(self.id));
        }

        self.data.relation_graph.add_edge(from, to, relation_type);

        propagate_from(self, &from)?;

        Ok(())
    }

    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()> {
        todo!()
    }

    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult<()> {
        self.data.relation_graph.remove_edge(from, to);

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
