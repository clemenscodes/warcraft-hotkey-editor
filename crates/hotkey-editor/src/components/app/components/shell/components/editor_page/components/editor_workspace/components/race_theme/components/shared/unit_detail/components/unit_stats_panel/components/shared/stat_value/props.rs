use super::super::stat_figure::StatFigure;
use super::view::StatValueView;
use dioxus::prelude::*;

/// A stat value leaf's input: the domain figure it presents. This is the default
/// value look every plain-figure row shares — tabular, right-aligned, dimmed when the
/// figure reports itself muted. A row whose value carries its own identity (hit
/// points, mana) renders its own span instead; every ordinary figure nests this leaf.
#[derive(Props, Clone, PartialEq)]
pub struct StatValueProps<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure + Clone> From<&StatValueView<Figure>> for StatValueProps<Figure> {
    fn from(view: &StatValueView<Figure>) -> Self {
        let StatValueView { value } = view.clone();
        Self { value }
    }
}

impl<Figure: StatFigure + Clone> ddd::Props for StatValueProps<Figure> {
    type View = StatValueView<Figure>;
}
