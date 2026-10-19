use super::super::stat_figure::StatFigure;
use super::view::StatRowView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatRowModel<Figure: StatFigure> {
    pub label: &'static str,
    pub value: Figure,
}

impl<Figure: StatFigure + Clone> From<&StatRowView<Figure>> for StatRowModel<Figure> {
    fn from(view: &StatRowView<Figure>) -> Self {
        let StatRowView { label, value } = view.clone();
        Self { label, value }
    }
}

impl<Figure: StatFigure + Clone> ddd::Layered for StatRowModel<Figure> {
    type Layer = ddd::PresentationLayer;
}

impl<Figure: StatFigure + Clone> ddd::Model for StatRowModel<Figure> {
    type View = StatRowView<Figure>;
}
