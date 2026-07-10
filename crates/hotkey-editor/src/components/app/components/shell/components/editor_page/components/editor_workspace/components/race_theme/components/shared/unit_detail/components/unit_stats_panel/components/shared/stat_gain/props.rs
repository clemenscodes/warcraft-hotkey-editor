use super::super::stat_figure::StatFigure;
use super::view::StatGainView;
use dioxus::prelude::*;

/// A stat gain leaf's input: the per-level growth figure it presents. This is the
/// default green gain look, sitting inline after a value and dimmed when the figure
/// reports itself muted. A row whose gain carries its own identity (a regeneration's
/// end-aligned placement, mana's blue) renders its own span instead.
#[derive(Props, Clone, PartialEq)]
pub struct StatGainProps<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure + Clone> From<&StatGainView<Figure>> for StatGainProps<Figure> {
    fn from(view: &StatGainView<Figure>) -> Self {
        let StatGainView { value } = view.clone();
        Self { value }
    }
}

impl<Figure: StatFigure + Clone> ddd::Props for StatGainProps<Figure> {
    type View = StatGainView<Figure>;
}
