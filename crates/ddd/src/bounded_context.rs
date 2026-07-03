/// A boundary within which one model and one ubiquitous language hold
/// consistently — strategic DDD's central unit.
///
/// The same word means different things in different contexts ("binding" to the
/// editor is not "binding" to the game's data pipeline), so each context keeps
/// its own model and translates at the edges. A bounded context usually maps to a
/// crate here. An intent marker: it labels the type that anchors a context.
pub trait BoundedContext {}
