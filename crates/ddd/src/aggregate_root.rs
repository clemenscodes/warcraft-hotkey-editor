use crate::DomainLayer;
use crate::Layered;

/// The single entry point to a cluster of domain objects that must stay mutually
/// consistent — the transactional boundary of the domain.
///
/// Every change to the objects inside the cluster goes through the root, so the
/// root is the one place invariants are enforced. A [`crate::Repository`] loads
/// and saves aggregates one root at a time, and a [`crate::Service`] mutates
/// exactly one aggregate root through named [`crate::Command`]s. `Clone` lets a
/// service snapshot the aggregate before persisting it; the aggregate lives, by
/// definition, in the domain layer.
pub trait AggregateRoot: Clone + Layered<Layer = DomainLayer> {}
