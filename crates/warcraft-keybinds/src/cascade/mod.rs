//! The cascade conflict-resolution algorithm: build a conflict graph from
//! the current `CustomKeys`, schedule anchor decisions in a queue, and emit
//! a plan of position changes that resolves cross-unit and intra-unit
//! collisions.
//!
//! See `crate::custom_keys::CustomKeys::resolve_conflicts` for the
//! user-triggered entry point.

pub mod conflict_graph;
pub mod planner;
pub mod queue;
