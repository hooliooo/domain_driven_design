use uuid::Uuid;

use crate::traits::value_object::ValueObject;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct CommandId {
    value: Uuid,
}

impl CommandId {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for CommandId {}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct EventId {
    value: Uuid,
}

impl EventId {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn new_random() -> Self {
        Self {
            value: uuid::Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for EventId {}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct RequestId {
    value: Uuid,
}

impl RequestId {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn new_random() -> Self {
        Self {
            value: uuid::Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for RequestId {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct IssuerId {
    value: String,
}

impl IssuerId {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl ValueObject for IssuerId {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct UserId {
    value: Uuid,
}

impl UserId {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl ValueObject for UserId {}
