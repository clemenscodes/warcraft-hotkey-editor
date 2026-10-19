use warcraft_api::HitPointsRegen;

#[derive(Clone, PartialEq)]
pub struct RegenConditionNoteView {
    pub regen: HitPointsRegen,
}

impl ddd::Layered for RegenConditionNoteView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegenConditionNoteView {}
