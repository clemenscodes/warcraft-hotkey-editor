use super::model::CollisionKindTabModel;

/// The tab as drawn. A tab names its kind by the first word of the breadcrumb
/// label ("Cross", "Intra", "Hotkey"): the page already says these are
/// collisions, and a third of a phone's width holds one word above its count.
pub(super) struct CollisionKindTabPresentation {
    pub(super) label: String,
}

impl ddd::Layered for CollisionKindTabPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for CollisionKindTabPresentation {
    type Model = CollisionKindTabModel;
}

impl From<&CollisionKindTabModel> for CollisionKindTabPresentation {
    fn from(model: &CollisionKindTabModel) -> Self {
        let full_label = model.label.as_str();
        let kind_word = full_label.split_whitespace().next().unwrap_or(full_label);
        let label = kind_word.to_string();
        Self { label }
    }
}
