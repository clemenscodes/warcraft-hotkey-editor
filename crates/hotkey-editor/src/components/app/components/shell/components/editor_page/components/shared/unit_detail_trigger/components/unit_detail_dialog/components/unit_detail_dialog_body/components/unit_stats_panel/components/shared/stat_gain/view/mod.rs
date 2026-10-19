use super::super::stat_figure::StatFigure;

#[derive(Clone, PartialEq)]
pub struct StatGainView<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure> ddd::Layered for StatGainView<Figure> {
    type Layer = ddd::PresentationLayer;
}

impl<Figure: StatFigure> ddd::View for StatGainView<Figure> {}
