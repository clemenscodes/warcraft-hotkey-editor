use super::super::stat_figure::StatFigure;
use super::components::active_stat_value::ActiveStatValueProps;
use super::components::muted_stat_value::MutedStatValueProps;
use super::props::StatValueProps;

impl<Figure: StatFigure> From<&StatValueProps<Figure>> for ActiveStatValueProps {
    fn from(props: &StatValueProps<Figure>) -> Self {
        let text = props.value.display();
        Self { text }
    }
}

impl<Figure: StatFigure> From<&StatValueProps<Figure>> for MutedStatValueProps {
    fn from(props: &StatValueProps<Figure>) -> Self {
        let text = props.value.display();
        Self { text }
    }
}
