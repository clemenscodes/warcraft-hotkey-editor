//! Collision detection and reporting — both per-unit (one command card's
//! conflicts) and cross-unit (positions where abilities collide across
//! every unit that carries them).

pub mod cross_unit;
pub(crate) mod island_partition;
pub mod summary;
pub mod unit_report;
