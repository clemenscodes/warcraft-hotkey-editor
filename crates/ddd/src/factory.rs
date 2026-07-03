use crate::DomainLayer;
use crate::Layered;

/// Encapsulates the creation of a complex domain object so it can never be
/// constructed in an invalid state.
///
/// When building an [`crate::AggregateRoot`] or [`crate::Entity`] takes more than
/// a struct literal — defaults to materialise, invariants to check, several
/// inputs to reconcile — that assembly belongs in a factory. Materialising a
/// fresh `CustomKeys` from the bundled default text and a grid layout is a
/// factory's job.
pub trait Factory<Product>: Layered<Layer = DomainLayer> {
    /// The inputs the factory needs to produce a valid product.
    type Blueprint;

    /// Produces a fully valid product from its blueprint.
    fn create(&self, blueprint: Self::Blueprint) -> Product;
}
