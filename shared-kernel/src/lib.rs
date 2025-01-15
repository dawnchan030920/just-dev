use std::marker::PhantomData;

use uuid::Uuid;

#[derive(Debug)]
pub struct Id<T> {
    pub id: Uuid,
    phantom: PhantomData<T>,
}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
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
