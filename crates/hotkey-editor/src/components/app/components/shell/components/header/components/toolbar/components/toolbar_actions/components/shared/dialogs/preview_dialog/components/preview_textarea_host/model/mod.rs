use super::view::PreviewTextareaHostView;

/// The preview host's ddd model. The component is connected — it sources the document from
/// context in its presentation builder — so this is the fieldless published contract the
/// frame's body region names as its `Render::Model`.
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
