use uuid::Uuid;

use crate::traits::value_object::ValueObject;

/// The unique identifier of the Command
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct CommandId {
    value: Uuid,
}

impl CommandId {
    /// Creates a new CommandId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    /// The unique identifier of the Command
    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for CommandId {}

/// The unique identifier of the Event
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct EventId {
    value: Uuid,
}

impl EventId {
    /// Creates a new EventId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    /// Creates a new, randomly generated EventId
    pub fn new_random() -> Self {
        Self {
            value: uuid::Uuid::new_v4(),
        }
    }

    /// The unique identifier of the Event
    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for EventId {}

/// The unique identifier of the Request
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct RequestId {
    value: Uuid,
}

impl RequestId {
    /// Creates a new RequestId
    /// # Arguments
    /// * `value` - The universally unique identifier (UUID)
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    /// Creates a new, randomly generated RequestId
    pub fn new_random() -> Self {
        Self {
            value: uuid::Uuid::new_v4(),
        }
    }

    /// The unique identifier of the Request
    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for RequestId {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct AuthorizedParty {
    value: String,
}

impl AuthorizedParty {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl ValueObject for AuthorizedParty {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct UserId<T> {
    value: T,
}

impl<Uuid> UserId<Uuid> {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for UserId<Uuid> {}
