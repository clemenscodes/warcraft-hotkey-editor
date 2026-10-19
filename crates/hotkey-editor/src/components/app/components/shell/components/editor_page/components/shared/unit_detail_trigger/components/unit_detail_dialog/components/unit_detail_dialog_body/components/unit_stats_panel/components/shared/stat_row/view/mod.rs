use super::super::stat_figure::StatFigure;

#[derive(Clone, PartialEq)]
pub struct StatRowView<Figure: StatFigure> {
    pub label: &'static str,
    pub value: Figure,
}

impl<Figure: StatFigure> ddd::Layered for StatRowView<Figure> {
    type Layer = ddd::PresentationLayer;
}

impl<Figure: StatFigure> ddd::View for StatRowView<Figure> {}
