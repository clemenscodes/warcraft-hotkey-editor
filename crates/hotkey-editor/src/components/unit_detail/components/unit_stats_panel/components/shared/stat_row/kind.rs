use super::state::StatRowVariant;
use dioxus::prelude::*;

/// The kind of stat a generic [`super::StatRow`] renders. A zero-sized marker, the
/// stat row's counterpart to [`crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridTileKind`]:
/// the base row encodes the label-plus-value-side shape once and is generic over
/// this kind, so it stays agnostic to which stat fills it. Each semantic row binds
/// its own kind — hit points carry a `u32`, armor an `f32`, the defense type a
/// `DefenseType` — and the base row just places whatever the kind produces.
///
/// The kind carries the DOMAIN value as its associated `Value`; the display string
/// is formatted at the leaves inside [`Self::cells`], never at the props boundary.
pub trait StatRowKind: Clone + PartialEq + Default + 'static {
    /// The domain value this row carries (e.g. `u32` hit points, `f32` armor,
    /// `DefenseType`), not a pre-formatted string.
    type Value: Clone + PartialEq + 'static;

    /// The row's label. A category label is genuine UI copy sourced from a `data.rs`
    /// const (e.g. `data::HIT_POINTS`); a label that is itself domain vocabulary — the
    /// hero attribute names — is sourced from the domain type's `Display`
    /// (`PrimaryAttribute::Strength.to_string()`), never re-typed in the renderer.
    fn label() -> String;

    /// The colour/size variant driving the hit-points and mana treatments.
    fn variant() -> StatRowVariant {
        StatRowVariant::Default
    }

    /// The variant as its `data-variant` attribute string. A provided method so the
    /// base row's body only names it, never chaining the enum-to-attribute mapping.
    fn variant_attribute() -> &'static str {
        let variant = Self::variant();
        variant.data_attr()
    }

    /// Whether this is a regeneration row (indented, gain-carrying).
    fn is_regen() -> bool {
        false
    }

    /// Whether this row is the hero's primary attribute (its glow treatment).
    fn is_primary(value: &Self::Value) -> bool {
        let _unused = value;
        false
    }

    /// Render the row's value-side content from its domain value, formatting the
    /// display string at the leaves (`StatRowValue`, `StatRowGain`, `RegenQualifier`
    /// under [`super::components`]). This mirrors `GridTileKind::tile` — the kind
    /// produces the value-side markup, the base row just places it.
    fn cells(value: Self::Value) -> Element;
}
