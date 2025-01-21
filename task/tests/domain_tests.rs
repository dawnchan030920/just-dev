use shared_kernel::{Entity, Id};
use task::domain::{
    error::TaskDomainError,
    list::{List, ListAggregateRoot},
    net::{Net, NetAggregateRoot, RelationType, Status},
    task::{Task, TaskAggregateRoot},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_aggregate_root() {
        let mut list = Entity::new("Test List".to_string());
        assert_eq!(list.data.title, "Test List");

        list.rename("Renamed List".to_string());
        assert_eq!(list.data.title, "Renamed List");
    }

    #[test]
    fn test_task_aggregate_root() {
        let list_id = Id::new();
        let mut task = Entity::new("Test Task".to_string(), list_id);
        assert_eq!(task.data.name, "Test Task");
        assert_eq!(task.data.list, list_id);

        task.rename("Renamed Task".to_string());
        assert_eq!(task.data.name, "Renamed Task");

        let new_list_id = Id::new();
        task.categorize_to(new_list_id);
        assert_eq!(task.data.list, new_list_id);
    }

    #[test]
    fn test_net_aggregate_root() {
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

        let status_id = net.data.schema.status[0].id;
        net.change_status_name(status_id, "Renamed Status".to_string())
            .unwrap();
        assert_eq!(net.data.schema.status[0].data.name, "Renamed Status");

        net.change_default(status_id).unwrap();
        assert_eq!(net.data.schema.default, status_id);

        let task_id = Id::new();
        net.add_task(task_id).unwrap();
        assert_eq!(net.data.tasks.len(), 1);
        assert_eq!(net.data.tasks[&task_id], status_id);

        net.change_task_status(task_id, net.data.schema.accepted)
            .unwrap();
        assert_eq!(net.data.tasks[&task_id], net.data.schema.accepted);

        let task_id_2 = Id::new();
        net.add_task(task_id_2).unwrap();
        net.new_relation(task_id, task_id_2, RelationType::Compose)
            .unwrap();
        assert!(net.data.relations.contains_edge(task_id, task_id_2));

        net.remove_relation(task_id, task_id_2).unwrap();
        assert!(!net.data.relations.contains_edge(task_id, task_id_2));

        net.remove_task(task_id).unwrap();
        assert!(!net.data.tasks.contains_key(&task_id));
    }

    #[test]
    fn test_task_domain_error() {
        let net_id = Id::new();
        let status_id = Id::new();
        let task_id = Id::new();

        let error = TaskDomainError::StatusNotFoundInNet {
            net: net_id,
            status: status_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("status {:?} not found in net {:?}", status_id, net_id)
        );

        let error = TaskDomainError::TaskNotFoundInNet {
            net: net_id,
            task: task_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("task {:?} not found in net {:?}", task_id, net_id)
        );

        let error = TaskDomainError::RelationNotFoundInNet {
            net: net_id,
            from: task_id,
            to: task_id,
        };
        assert_eq!(
            format!("{}", error),
            format!(
                "relation from {:?} to {:?} not found in net {:?}",
                task_id, task_id, net_id
            )
        );

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

        let error = TaskDomainError::TaskAlreadyInNet {
            task: task_id,
            net: net_id,
        };
        assert_eq!(
            format!("{}", error),
            format!("task {:?} already in net {:?}", task_id, net_id)
        );

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

        let error = TaskDomainError::CycleNotAllowedInNet(net_id);
        assert_eq!(
            format!("{}", error),
            format!("cycle found in net {:?}", net_id)
        );
    }
}
