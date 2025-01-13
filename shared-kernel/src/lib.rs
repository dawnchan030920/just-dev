use std::marker::PhantomData;

use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id<T> {
    pub id: Uuid,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            phantom: PhantomData,
        }
    }
}

pub struct Entity<T> {
    pub id: Id<T>,
    pub data: T,
}

impl<T> From<T> for Entity<T> {
    fn from(value: T) -> Self {
        Self {
            id: Id::new(),
            data: value,
        }
    }
}
