use super::super::super::stat_figure::StatFigure;
use dioxus::prelude::*;

/// A stat row's value (e.g. the hit-points figure), a domain figure that presents
/// itself. Always present — the semantic row renders it only where a value belongs.
/// The figure decides its own muted state, so no parallel `is_zero` prop is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowValueProps<Figure: StatFigure> {
    pub value: Figure,
}
