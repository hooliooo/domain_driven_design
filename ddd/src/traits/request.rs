use std::{collections::HashSet, hash::Hash};

use chrono::{DateTime, Utc};

use crate::{
    enums::environment::Environment,
    structs::{ids::UserId, role::Role},
};

///
/// A Request is a Command that mutates an Aggregate or a Query that returns data. The trait defines the required metadata in a
/// Command-Query Responsibility Segregation (CQRS) architecture
/// ```
///
/// use ddd::AuthenticatedRequest;
/// use ddd::traits::request::Request;
/// use ddd::traits::request::AuthenticatedRequest;
/// use ddd::structs::ids::RequestId;
/// use ddd::structs::ids::UserId;
/// use ddd::structs::role::Role;
/// use chrono::DateTime;
/// use chrono::Utc;
/// use std::collections::HashSet;
///
/// #[derive(ddd::AuthenticatedRequest, Debug)]
/// pub struct CreateAccount {
///     request_id: RequestId,
///     issuer_id: (),
///     environment: ddd::enums::environment::Environment,
///     issued_at: DateTime<Utc>,
///     user_id: UserId,
///     roles: HashSet<Role>
/// }
///
/// impl CreateAccount {
///     pub fn new(request_id: uuid::Uuid) -> Self {
///         Self {
///             request_id: RequestId::new(request_id),
///             issuer_id: (),
///             environment: ddd::enums::environment::Environment::Development,
///             issued_at: Utc::now(),
///             user_id: UserId::new(uuid::Uuid::new_v4()),
///             roles: HashSet::default()
///         }
///     }
/// }
///
/// let a = CreateAccount::new(uuid::Uuid::new_v4());
/// let b = CreateAccount::new(uuid::Uuid::new_v4());
///
/// let c = CreateAccount::new(uuid::Uuid::new_v4());
/// let d = CreateAccount::new(uuid::Uuid::new_v4());
/// ```
pub trait Request {
    type RequestId: Eq + PartialEq + Hash + Clone;
    /// The unique identifier of the Request
    fn request_id(&self) -> &Self::RequestId;

    type IssuerId: Eq + PartialEq + Hash + Clone;
    /// The identifier of the client that issued the command
    fn issuer_id(&self) -> &Self::IssuerId;

    fn environment(&self) -> &Environment;

    fn issued_at(&self) -> &DateTime<Utc>;
}

pub trait AuthenticatedRequest: Request {
    /// The user identifier
    fn user_id(&self) -> &UserId;

    /// The roles of the user
    fn roles(&self) -> &HashSet<Role>;
}
