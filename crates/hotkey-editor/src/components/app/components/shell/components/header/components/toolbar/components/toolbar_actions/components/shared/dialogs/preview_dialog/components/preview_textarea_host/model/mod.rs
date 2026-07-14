use super::view::PreviewTextareaHostView;

#[derive(Clone, PartialEq, Default)]
pub struct PreviewTextareaHostModel;

impl From<&PreviewTextareaHostView> for PreviewTextareaHostModel {
    fn from(_view: &PreviewTextareaHostView) -> Self {
        Self
    }
}

impl ddd::Model for PreviewTextareaHostModel {
    type View = PreviewTextareaHostView;
}
