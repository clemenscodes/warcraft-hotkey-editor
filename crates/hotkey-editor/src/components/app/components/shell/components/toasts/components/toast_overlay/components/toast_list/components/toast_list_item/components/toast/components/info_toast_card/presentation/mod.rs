use super::model::InfoToastCardModel;

/// The info card's presentation: the shared-card fields shaped from the toast
/// record. Built purely from the model — a shaping leaf, no effects.
pub struct InfoToastCardPresentation {
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) id: usize,
}

impl From<&InfoToastCardModel> for InfoToastCardPresentation {
    fn from(model: &InfoToastCardModel) -> Self {
        let title = model.record.title().to_string();
        let description = model.record.description();
        let id = model.record.id();
        Self {
            title,
            description,
            id,
        }
    }
}

impl ddd::Presentation for InfoToastCardPresentation {
    type Model = InfoToastCardModel;
}
