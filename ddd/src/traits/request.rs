use std::hash::Hash;

use chrono::{DateTime, Utc};

use crate::enums::environment::Environment;

/// ```
/// A Request is a Command that mutates an Aggregate or a Query that returns data. The trait defines the required metadata in a
/// Command-Query Responsibility Segregation (CQRS) architecture
///
/// #![cfg(feature = "derive")]
/// use ddd::Request;
/// use ddd::traits::request::Request;
/// use ddd::structs::ids::RequestId;
/// use chrono::DateTime;
/// use chrono::Utc;
///
/// #[derive(ddd::Request, Debug)]
/// pub struct CreateAccount {
///     request_id: RequestId,
///     environment: ddd::enums::environment::Environment,
///     issuer_id: (),
///     issued_at: DateTime<Utc>
/// }
///
/// impl CreateAccount {
///     pub fn new(request_id: uuid::Uuid) -> Self {
///         Self {
///             request_id: RequestId::new(request_id),
///             environment: ddd::enums::environment::Environment::Development,
///             issuer_id: (),
///             issued_at: Utc::now()
///         }
///     }
/// }
/// let a = CreateAccount::new(uuid::Uuid::new_v4());
/// let b = CreateAccount::new(uuid::Uuid::new_v4());
///
/// let c = CreateAccount::new(uuid::Uuid::new_v4());
/// let d = CreateAccount::new(uuid::Uuid::new_v4());
/// ```
pub trait Request<'a> {
    type RequestId: Eq + PartialEq + Hash + Clone;
    /// The unique identifier of the Request
    fn request_id(&'a self) -> &'a Self::RequestId;

    type IssuerId: Eq + PartialEq + Hash + Clone;
    /// The identifier of the issuer of the command
    fn issuer_id(&'a self) -> &'a Self::IssuerId;

    fn environment(&'a self) -> &'a Environment;

    fn issued_at(&'a self) -> &'a DateTime<Utc>;
}
