use std::hash::Hash;

use uuid::Uuid;

use crate::building_blocks::value_object::ValueObject;

/// The AggregateId is the base trait that types should use when acting as the identity type for an Aggregate
pub trait AggregateId: Copy + Clone + Eq + PartialEq + Hash {}

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
