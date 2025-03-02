use std::hash::Hash;

/// The AggregateRootId is the base trait that types should use when acting as the identity type for an Aggregate
pub trait AggregateRootId: Clone + Eq + PartialEq + Hash {}
