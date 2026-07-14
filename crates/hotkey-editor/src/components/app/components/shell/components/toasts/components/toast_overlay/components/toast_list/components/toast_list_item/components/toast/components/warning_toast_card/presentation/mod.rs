use super::model::WarningToastCardModel;

pub struct WarningToastCardPresentation {
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) id: usize,
}

impl From<&WarningToastCardModel> for WarningToastCardPresentation {
    fn from(model: &WarningToastCardModel) -> Self {
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

impl ddd::Presentation for WarningToastCardPresentation {
    type Model = WarningToastCardModel;
}
