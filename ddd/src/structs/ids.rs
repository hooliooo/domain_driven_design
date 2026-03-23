use std::hash::Hash;

use uuid::Uuid;

use crate::traits::{aggregate_id::AggregateId, value_object::ValueObject};

/// The unique identifier of the Command
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct CommandId(Uuid);

impl CommandId {
    /// Creates a new CommandId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// The unique identifier of the Command
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl ValueObject for CommandId {}

/// The unique identifier of the Event
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct EventId(Uuid);

impl EventId {
    /// Creates a new EventId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Creates a new, randomly generated EventId
    pub fn new_random_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// The unique identifier of the Event
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl ValueObject for EventId {}

/// The unique identifier of the Request
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct RequestId(Uuid);

impl RequestId {
    /// Creates a new RequestId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Creates a new, randomly generated RequestId
    pub fn new_random_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// The unique identifier of the Request
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl ValueObject for RequestId {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct AuthorizedParty(String);

impl AuthorizedParty {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl ValueObject for AuthorizedParty {}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct UserId<T>(T);

impl<T> UserId<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for UserId<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> AggregateId for UserId<T> where T: Copy + Clone + Eq + PartialEq + Hash {}
impl<T> ValueObject for UserId<T> where T: Clone + Eq + PartialEq + Hash {}
