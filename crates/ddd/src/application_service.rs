use crate::ApplicationLayer;
use crate::Layered;

/// An application-layer orchestrator for a use case: it drives the domain and its
/// persistence but holds no business rules of its own.
///
/// The functional shape of an application service is [`crate::Service`], which
/// supplies the write-through commit and dispatch machinery. This marker adds the
/// role and layer classification — "this type is application-layer, not domain" —
/// so seams can accept or reject it by layer and so it is greppable. Because it
/// requires the application layer and [`crate::DomainService`] requires the
/// domain layer, no type can be both.
pub trait ApplicationService: Layered<Layer = ApplicationLayer> {}
