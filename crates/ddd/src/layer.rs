mod sealed {
    pub trait Sealed {}
}

/// A tag identifying one architectural layer. Sealed: the set of layers is
/// closed — [`DomainLayer`], [`ApplicationLayer`], [`InfrastructureLayer`], and
/// [`PresentationLayer`] are the only ones, and no downstream crate can invent a
/// fifth.
pub trait Layer: sealed::Sealed {}

/// The Domain layer: the model itself — entities, value objects, aggregate
/// roots, domain services, and domain events. Depends on nothing outside itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct DomainLayer;

/// The Application layer: use-case orchestration — application services and the
/// commands and queries they run. Depends on the domain, holds no business rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ApplicationLayer;

/// The Infrastructure layer: adapters to the outside world — repository and
/// event-bus implementations, persistence, framework glue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct InfrastructureLayer;

/// The Presentation layer: the user-facing edge — the renderer, its components,
/// and view state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PresentationLayer;

impl sealed::Sealed for DomainLayer {}
impl sealed::Sealed for ApplicationLayer {}
impl sealed::Sealed for InfrastructureLayer {}
impl sealed::Sealed for PresentationLayer {}

impl Layer for DomainLayer {}
impl Layer for ApplicationLayer {}
impl Layer for InfrastructureLayer {}
impl Layer for PresentationLayer {}

/// Declares the single layer a type belongs to.
///
/// A type has exactly one home layer, because it can carry only one `Layered`
/// implementation — coherence forbids a second. That is what makes layer
/// membership *mutually exclusive* without any negative trait bound (which Rust
/// does not have): a second `impl Layered` for the same type with a different
/// `Layer` is a conflicting implementation and does not compile. A boundary
/// rejects the wrong layer by binding the associated type:
/// `where Type: Layered<Layer = DomainLayer>`.
///
/// # A boundary accepts only its layer
///
/// ```
/// use ddd::{Layered, DomainLayer};
/// struct Binding;
/// impl Layered for Binding {
///     type Layer = DomainLayer;
/// }
/// fn only_domain<Type: Layered<Layer = DomainLayer>>() {}
/// only_domain::<Binding>();
/// ```
///
/// # Two layers on one type do not compile
///
/// ```compile_fail
/// use ddd::{ApplicationLayer, DomainLayer, Layered};
/// struct CustomKeysService;
/// impl Layered for CustomKeysService {
///     type Layer = DomainLayer;
/// }
/// impl Layered for CustomKeysService {
///     type Layer = ApplicationLayer;
/// }
/// ```
pub trait Layered {
    /// The one layer this type lives in.
    type Layer: Layer;
}
