use crate::enums::environment::Environment;
use std::hash::Hash;

///
/// ```
/// #![cfg(feature = "derive")]
///
/// use ddd::DomainEvent;
/// use ddd::traits::domain_event::DomainEvent;
/// use ddd::structs::ids::CommandId;
/// use ddd::structs::ids::EventId;
/// use chrono::DateTime;
/// use chrono::Utc;
///
/// #[derive(ddd::DomainEvent, Debug)]
/// pub struct CreatedAccount {
///     command_id: CommandId,
///     environment: ddd::enums::environment::Environment,
///     event_id: EventId,
///     issuer_id: (),
///     issued_at: DateTime<Utc>
/// }
///
/// impl CreatedAccount {
///     pub fn new(command_id: uuid::Uuid) -> Self {
///         Self {
///             command_id: CommandId::new(command_id),
///             environment: ddd::enums::environment::Environment::Development,
///             event_id: EventId::new_random(),
///             issuer_id: (),
///             issued_at: Utc::now()
///         }
///     }
/// }
///
/// let a = CreatedAccount::new(uuid::Uuid::new_v4());
/// let b = CreatedAccount::new(uuid::Uuid::new_v4());
///
///
/// let c = CreatedAccount::new(uuid::Uuid::new_v4());
/// let d = CreatedAccount::new(uuid::Uuid::new_v4());
///
/// ```
pub trait DomainEvent {
    type CommandId: Eq + PartialEq + Hash + Copy + Clone;
    /// The unique identifier of the Command that created this Domain event
    fn command_id(&self) -> &Self::CommandId;

    type EventId: Eq + PartialEq + Hash + Copy + Clone;
    /// The unique identifier of the Domain Event
    fn event_id(&self) -> &Self::EventId;

    type IssuerId: Eq + PartialEq + Hash + Copy + Clone;
    /// The identifier of the issuer of the command
    fn issuer_id(&self) -> &Self::IssuerId;

    fn issued_at(&self) -> &chrono::DateTime<chrono::Utc>;

    fn environment(&self) -> &Environment;
}
