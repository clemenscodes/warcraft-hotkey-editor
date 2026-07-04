use super::super::super::stat_figure::StatFigure;
use dioxus::prelude::*;

/// A stat row's per-level or regen gain (e.g. "+2.5"), a domain figure that presents
/// itself. Always present where a row carries a gain; the semantic row renders it
/// only in that position. The figure decides its own muted state (a regen or growth
/// of zero), so no parallel `is_zero` prop is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowGainProps<Figure: StatFigure> {
    pub value: Figure,
}
