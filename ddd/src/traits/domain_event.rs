use crate::{
    enums::environment::Environment,
    structs::ids::{CommandId, EventId, IssuerId},
};

///
/// ```
///
///
/// use ddd::DomainEvent;
/// use ddd::traits::domain_event::DomainEvent;
/// use ddd::structs::ids::CommandId;
/// use ddd::structs::ids::EventId;
/// use ddd::structs::ids::IssuerId;
/// use chrono::DateTime;
/// use chrono::Utc;
/// use uuid::Uuid;
/// use std::any::Any;
///
/// #[derive(ddd::DomainEvent, Debug)]
/// pub struct CreatedAccount {
///     command_id: CommandId,
///     environment: ddd::enums::environment::Environment,
///     event_id: EventId,
///     issuer_id: IssuerId,
///     issued_at: DateTime<Utc>
/// }
///
/// impl CreatedAccount {
///     pub fn new(command_id: uuid::Uuid) -> Self {
///         Self {
///             command_id: CommandId::new(command_id),
///             environment: ddd::enums::environment::Environment::Development,
///             event_id: EventId::new_random(),
///             issuer_id: IssuerId::new("test.client".to_string()),
///             issued_at: Utc::now()
///         }
///     }
/// }
///
/// let a = CreatedAccount::new(uuid::Uuid::new_v4());
/// let b = CreatedAccount::new(uuid::Uuid::new_v4());
/// let boxed_a = Box::new(a);
/// let unboxed_a_as_any = boxed_a.as_any();
/// assert!(unboxed_a_as_any.is::<CreatedAccount>());
/// assert!(unboxed_a_as_any.downcast_ref::<CreatedAccount>().is_some());
///
/// let c = CreatedAccount::new(uuid::Uuid::new_v4());
/// let d = CreatedAccount::new(uuid::Uuid::new_v4());
///
/// ```
pub trait DomainEvent {
    /// The unique identifier of the Command that created this Domain event
    fn command_id(&self) -> &CommandId;

    /// The unique identifier of the Domain Event
    fn event_id(&self) -> &EventId;

    /// The identifier of the client that issued the DomainEvent
    fn issuer_id(&self) -> &IssuerId;

    fn issued_at(&self) -> &chrono::DateTime<chrono::Utc>;

    fn environment(&self) -> &Environment;

    fn as_any(&self) -> &dyn std::any::Any;
}

// #[cfg(test)]
// mod test {
//
//     use chrono::{DateTime, Utc};
//     use uuid::Uuid;
//
//     use crate::DomainEvent;
//     use crate::enums::environment::Environment;
//     use crate::structs::ids::{CommandId, EventId};
//
//     #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
//     struct IssuerId {
//         id: Uuid,
//     }
//
//     #[derive(Debug, DomainEvent)]
//     struct CreateAccount {
//         command_id: CommandId,
//         event_id: EventId,
//         issuer_id: IssuerId,
//         environment: Environment,
//         issued_at: DateTime<Utc>,
//     }
// }
