use super::super::stat_figure::StatFigure;
use super::view::StatValueView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatValueModel<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure + Clone> From<&StatValueView<Figure>> for StatValueModel<Figure> {
    fn from(view: &StatValueView<Figure>) -> Self {
        let StatValueView { value } = view.clone();
        Self { value }
    }
}

impl<Figure: StatFigure + Clone> ddd::Model for StatValueModel<Figure> {
    type View = StatValueView<Figure>;
}
