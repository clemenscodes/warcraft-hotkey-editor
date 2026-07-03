use crate::DomainLayer;
use crate::Layered;

/// A domain object defined by a stable identity rather than by its attributes.
///
/// Two entities are the same entity when their [`Entity::Identity`] is equal,
/// even if every other field differs — a rebound hotkey in the same slot is
/// still the same slot. This is the contrast with a [`crate::ValueObject`], which
/// has no identity and is compared by value.
///
/// `Identity` is bounded only by `Eq` here; consequent DDD would tighten it to
/// [`crate::Identifier`] so an id is always a value object, at the cost of
/// forcing every entity to wrap its identity in a named type.
pub trait Entity: Layered<Layer = DomainLayer> {
    /// The type that uniquely identifies this entity within its aggregate.
    type Identity: Eq;

    /// Returns the stable identity of this entity.
    fn identity(&self) -> &Self::Identity;
}
