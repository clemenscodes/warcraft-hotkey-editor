use crate::ValueObject;

/// The value object that serves as an [`crate::Entity`]'s identity.
///
/// An identifier is itself a value object — immutable, compared by value — whose
/// job is to say *which* entity this is: a `BindingId`, a `UnitId`. Modelling
/// identity as a distinct type (rather than a bare `u32`) is what stops one
/// entity's id from being silently used where another's is expected.
pub trait Identifier: ValueObject {}
