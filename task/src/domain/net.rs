use std::collections::HashMap;

use bimap::BiHashMap;
use daggy::{
    petgraph::{algo::toposort, csr::DefaultIx, data::DataMap, graph::DiGraph},
    Dag, NodeIndex, Walker,
};
use shared_kernel::{Entity, Id};

use super::{error::TaskDomainError, task::Task};

#[derive(Debug)]
pub struct Net {
    relation_graph: RelationGraph,
    schema: Schema,
    tasks: HashMap<Id<Task>, Id<Status>>,
}

#[derive(Debug)]
struct RelationGraph {
    relations: Dag<Id<Task>, RelationType>,
    task_index: BiHashMap<Id<Task>, NodeIndex<DefaultIx>>,
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

impl RelationGraph {
    fn add_task(&mut self, task: Id<Task>) {
        let id = self.relations.add_node(task);
        self.task_index.insert(task, id);
    }
}

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

fn propagate_from(net: &mut Entity<Net>, task: &Id<Task>) -> TaskDomainResult<()> {
    let tasks_after = toposort(&net.data.relation_graph.relations, None)
        .unwrap()
        .into_iter()
        .map(|ix| {
            net.data
                .relation_graph
                .task_index
                .get_by_right(&ix)
                .unwrap()
        })
        .skip_while(|t| **t != *task)
        .skip(1);

    for task in tasks_after {
        if let Some(accepted) = is_controlled_task_accepted(net, task)? {
            let stored_task_status =
                net.data
                    .tasks
                    .get_mut(task)
                    .ok_or(TaskDomainError::TaskNotFoundInNet {
                        net: net.id,
                        task: *task,
                    })?;

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
    let parents = net
        .data
        .relation_graph
        .relations
        .parents(
            *net.data
                .relation_graph
                .task_index
                .get_by_left(&task)
                .ok_or(TaskDomainError::TaskNotFoundInNet {
                    net: net.id,
                    task: *task,
                })?,
        )
        .iter(&net.data.relation_graph.relations);

    let mut have_subtasks = false;
    for (edge, node) in parents {
        let relation_type = net.data.relation_graph.relations.edge_weight(edge).unwrap();
        let task_id = net.data.relation_graph.relations.node_weight(node).unwrap();

        if !(*net
            .data
            .tasks
            .get(task_id)
            .ok_or(TaskDomainError::TaskNotFoundInNet {
                net: net.id,
                task: *task_id,
            })?
            == net.data.schema.accepted)
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
        self.data.relation_graph.add_task(task_id);
        Ok(())
    }

    fn change_task_status(
        &mut self,
        task_id: Id<Task>,
        status_id: Id<Status>,
    ) -> TaskDomainResult<()> {
        todo!()
    }

    fn new_relation(
        &mut self,
        from: Id<Task>,
        to: Id<Task>,
        relation_type: RelationType,
    ) -> TaskDomainResult<()> {
        todo!()
    }

    fn remove_task(&mut self, task_id: Id<Task>) -> TaskDomainResult<()> {
        todo!()
    }

    fn remove_relation(&mut self, from: Id<Task>, to: Id<Task>) -> TaskDomainResult<()> {
        todo!()
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
