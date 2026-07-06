use super::props::ToastTitleProps;
use super::style;
use tw_macro::ClassList;

/// The title's presentation: the type-tinted class plus the title text. The color
/// override per type is chosen here, never in the body.
pub struct ToastTitlePresentation {
    pub class: ClassList,
    pub title: String,
}

impl From<&ToastTitleProps> for ToastTitlePresentation {
    fn from(props: &ToastTitleProps) -> Self {
        let class = style::class(props.toast_type);
        let title = props.title.clone();
        Self { class, title }
    }
}
