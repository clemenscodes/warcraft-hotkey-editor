use crate::DomainLayer;
use crate::Layered;

/// Stateless domain logic that belongs to no single [`crate::Entity`] or
/// [`crate::ValueObject`] because it operates across several of them.
///
/// Collision detection across all bindings, cascade resolution — logic that
/// answers a domain question spanning many objects and owns no state of its own.
/// Contrast with a [`crate::Service`] (application layer: orchestration and
/// persistence, no rules): a domain service *is* the rule.
///
/// An intent marker that forces only its layer. Because it requires the domain
/// layer and [`crate::ApplicationService`] requires the application layer, one
/// type can never be both — the two layers are mutually exclusive.
pub trait DomainService: Layered<Layer = DomainLayer> {}
