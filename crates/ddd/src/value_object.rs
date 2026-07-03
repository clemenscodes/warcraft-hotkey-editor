use crate::DomainLayer;
use crate::Layered;

/// A domain object defined entirely by its attributes, with no identity.
///
/// Value objects are immutable and compared structurally: two value objects with
/// equal fields are interchangeable. Anything that answers "what" rather than
/// "which one" — a `GridCoordinate`, a `Hotkey` — is a value object. The
/// supertraits make the contract mechanical: it can be cloned, compared for
/// equality, and it lives in the domain layer.
pub trait ValueObject: Clone + Eq + Layered<Layer = DomainLayer> {}
