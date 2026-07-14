use super::super::stat_figure::StatFigure;

#[derive(Clone, PartialEq)]
pub struct StatValueView<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure> ddd::View for StatValueView<Figure> {}
