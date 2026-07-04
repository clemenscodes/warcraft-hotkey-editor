use super::kind::StatRowKind;
use dioxus::prelude::*;

/// The base stat row's input: the row's DOMAIN value, bound to a concrete
/// [`StatRowKind`] through the turbofish (`StatRow::<HitPointsKind>`). `StatRow`
/// encodes the label-plus-value-side shape once and is generic over the kind, so it
/// stays agnostic to which stat fills it; the bound kind supplies the label and the
/// variant and renders the value from its domain type. The kind never appears as a
/// field — it is a zero-sized marker named only at the call site, and `B::Value`
/// carries it into the props.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowProps<B: StatRowKind> {
    pub value: B::Value,
}
