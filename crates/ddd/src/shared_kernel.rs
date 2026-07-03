/// A model deliberately shared between two [`crate::BoundedContext`]s, which both
/// agree to change only together.
///
/// A shared kernel trades autonomy for less duplication: the shared subset must
/// stay agreed. This very crate is a shared kernel — the DDD vocabulary every
/// context speaks. An intent marker for the types that make up such a shared
/// core.
pub trait SharedKernel {}
