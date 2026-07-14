use super::super::stat_figure::StatFigure;
use super::view::StatGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatGainModel<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure + Clone> From<&StatGainView<Figure>> for StatGainModel<Figure> {
    fn from(view: &StatGainView<Figure>) -> Self {
        let StatGainView { value } = view.clone();
        Self { value }
    }
}

impl<Figure: StatFigure + Clone> ddd::Model for StatGainModel<Figure> {
    type View = StatGainView<Figure>;
}
