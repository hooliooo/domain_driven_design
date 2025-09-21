/// An Aggregate is a cluster of domain objects (entities and value objects) that are treated as a
/// single unit.
///
/// ```
/// use ddd::Aggregate;
/// use ddd::traits::aggregate::Aggregate;
/// use ddd::traits::entity::Entity;
/// use uuid::Uuid;
///
/// #[derive(ddd::Aggregate, Debug)]
/// pub struct Tenant {
///     #[generate_id(Uuid)]
///     #[entity_id]
///     id: TenantId,
///     #[field]
///     name: String,
///     #[field]
///     display_name: String,
///     version: u32
/// }
///
/// impl Tenant {
///     pub fn new(id: uuid::Uuid, name: String, display_name: String, version: u32) -> Self {
///         Self {
///             id: TenantId::new(id),
///             name,
///             display_name,
///             version
///         }
///     }
/// }
///
/// let a = Tenant::new(uuid::Uuid::new_v4(),"tenant.a".to_string(), "Tenant A".to_string(), 1);
/// let b = Tenant::new(uuid::Uuid::new_v4(),"tenant.b".to_string(), "Tenant B".to_string(), 2);
///
/// assert_ne!(a, b);
/// assert_eq!(Tenant::type_name(), "tenant");
/// assert_eq!(a.version(), 1);
/// assert_eq!(b.version(), 2);
/// ```
///
pub trait Aggregate {
    /// The current version of the Aggregate. Whenever the Aggregate changes, the version should be
    /// incremented to reflect an update has occured. The persistence layer should be the one
    /// responsible for incrementing an Aggregate's version
    fn version(&self) -> u32;

    fn type_name() -> &'static str;
}
