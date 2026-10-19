use super::model::SuccessToastCardModel;

pub struct SuccessToastCardPresentation {
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) id: usize,
}

impl From<&SuccessToastCardModel> for SuccessToastCardPresentation {
    fn from(model: &SuccessToastCardModel) -> Self {
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

impl ddd::Layered for SuccessToastCardPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for SuccessToastCardPresentation {
    type Model = SuccessToastCardModel;
}
