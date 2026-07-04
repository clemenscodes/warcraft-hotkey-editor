use super::props::ToastCardProps;
use super::style;
use crate::components::app::components::shell::components::toasts::ToastType;
use crate::styling::ClassList;

/// The card's presentation: the type-selected class and the `data-type` attribute
/// value. Shaped from the record's toast type; the body only places it.
pub struct ToastCardPresentation {
    pub class: ClassList,
    pub data_type: &'static str,
}

impl From<&ToastCardProps> for ToastCardPresentation {
    fn from(props: &ToastCardProps) -> Self {
        let toast_type: ToastType = props.record.toast_type();
        let class = style::class(toast_type);
        let data_type = toast_type.data_type();
        Self { class, data_type }
    }
}
