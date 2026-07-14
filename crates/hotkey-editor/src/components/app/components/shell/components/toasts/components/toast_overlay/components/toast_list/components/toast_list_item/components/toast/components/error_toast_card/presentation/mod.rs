use super::model::ErrorToastCardModel;

pub struct ErrorToastCardPresentation {
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) id: usize,
}

impl From<&ErrorToastCardModel> for ErrorToastCardPresentation {
    fn from(model: &ErrorToastCardModel) -> Self {
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

impl ddd::Presentation for ErrorToastCardPresentation {
    type Model = ErrorToastCardModel;
}
