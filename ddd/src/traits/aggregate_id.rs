use std::hash::Hash;

/// The AggregateId is the base trait that types should use when acting as the identity type for an Aggregate
pub trait AggregateId: Copy + Clone + Eq + PartialEq + Hash {}
