use crate::InfrastructureLayer;
use crate::Layered;

/// A concrete infrastructure implementation of a [`crate::Port`] — the "adapters"
/// of Ports & Adapters.
///
/// Where a port is the interface the application depends on, an adapter is the
/// thing on the other side of the boundary that fulfils it: a localStorage-backed
/// [`crate::Repository`], a `use_coroutine`-backed [`crate::EventBus`]. Adapters
/// live in the infrastructure layer, which this marker enforces.
pub trait Adapter: Layered<Layer = InfrastructureLayer> {}
