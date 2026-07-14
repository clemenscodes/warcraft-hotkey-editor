/// The published `View` contract mirroring [`UnbindableNoteModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnbindableNoteView {
    pub text: String,
}

impl ddd::View for UnbindableNoteView {}
