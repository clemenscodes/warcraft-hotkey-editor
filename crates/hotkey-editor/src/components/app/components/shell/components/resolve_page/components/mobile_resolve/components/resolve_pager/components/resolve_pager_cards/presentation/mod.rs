use super::model::ResolvePagerCardsModel;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveView, UnresolvedView,
};

pub(super) struct ResolvePagerCardsPresentation {
    pub(super) moves: Vec<MoveView>,
    pub(super) unresolved: Vec<UnresolvedView>,
    pub(super) is_clear: bool,
}

impl From<&ResolvePagerCardsModel> for ResolvePagerCardsPresentation {
    fn from(props: &ResolvePagerCardsModel) -> Self {
        let moves = match &props.section {
            Some(section) => section.moves().to_vec(),
            None => Vec::new(),
        };
        let unresolved = props.unresolved.clone();
        let is_clear = moves.is_empty() && unresolved.is_empty();
        Self {
            moves,
            unresolved,
            is_clear,
        }
    }
}

impl ddd::Layered for ResolvePagerCardsPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for ResolvePagerCardsPresentation {
    type Model = ResolvePagerCardsModel;
}
