use shared_kernel::Entity;

pub struct List {
    title: String,
}

pub trait ListAggregateRoot {}

impl ListAggregateRoot for Entity<List> {}
