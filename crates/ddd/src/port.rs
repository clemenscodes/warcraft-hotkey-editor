/// A boundary interface the application defines and the outside world plugs into
/// — the "ports" of Ports & Adapters (hexagonal architecture).
///
/// In Rust a port *is* a trait: [`crate::Repository`] and [`crate::EventBus`] are
/// ports already. This marker is a pragmatic label for the concept, with an
/// honest limitation — "this trait is a port" is a property of a *trait*, and
/// Rust markers attach to *types*, so it cannot be enforced the way a type's
/// layer can. It documents the hexagonal role; the crate graph does the
/// enforcing.
pub trait Port {}

/// A driving (primary) port: the API the application offers to the world that
/// drives it — the use cases a [`crate::Service`] exposes, invoked by the
/// presentation layer.
pub trait DrivingPort: Port {}

/// A driven (secondary) port: what the application needs *from* the world —
/// persistence ([`crate::Repository`]), messaging ([`crate::EventBus`]) — and
/// which infrastructure supplies as an [`crate::Adapter`].
pub trait DrivenPort: Port {}
