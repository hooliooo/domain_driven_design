use std::hash::Hash;

/// An **Entity** is an object whose equality is defined by its identity not by its attributes.
///
/// Two entities are the same if their identities are the same, even if their
/// attributes are different.
///
/// ```
///
///
/// use ddd::Entity;
/// use ddd::traits::entity::Entity;
///
/// #[derive(ddd::Entity, Debug)]
/// pub struct Person {
///     #[entity_id(u32)]
///     id: u32,
///     name: String,
/// }
///
/// impl Person {
///     pub fn new(id: u32, name: String) -> Self {
///         Self { id, name }
///     }
/// }
///
/// let a = Person::new(1, "Tom".to_string());
/// let b = Person::new(1, "Jerry".to_string());
///
/// assert_eq!(a, b);
///
/// let c = Person::new(2, "Tom".to_string());
/// let d = Person::new(3, "Tom".to_string());
///
/// assert_ne!(c, d);
/// ```
pub trait Entity: Eq + PartialEq + Hash {
    type Id: Clone + PartialEq + Hash + Send + Sync;

    fn id(&self) -> &Self::Id;
}
