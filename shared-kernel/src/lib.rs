use std::marker::PhantomData;

use uuid::Uuid;

/// A unique identifier for a data value object of type `T`.
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
        Some(self.cmp(other))
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
        *self
    }
}

impl<T> Id<T> {
    /// Creates a new unique identifier.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            phantom: PhantomData,
        }
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// An entity with a unique identifier and associated data value object.
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
