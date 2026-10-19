use super::view::RegenConditionNoteView;
use dioxus::prelude::*;
use warcraft_api::HitPointsRegen;

#[derive(Props, Clone, PartialEq)]
pub struct RegenConditionNoteModel {
    pub regen: HitPointsRegen,
}

impl From<&RegenConditionNoteView> for RegenConditionNoteModel {
    fn from(view: &RegenConditionNoteView) -> Self {
        let RegenConditionNoteView { regen } = view.clone();
        Self { regen }
    }
}

impl ddd::Layered for RegenConditionNoteModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RegenConditionNoteModel {
    type View = RegenConditionNoteView;
}
