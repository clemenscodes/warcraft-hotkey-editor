use super::data::{TOAST_ICON_ERROR, TOAST_ICON_INFO, TOAST_ICON_SUCCESS, TOAST_ICON_WARNING};
use super::props::ToastIconProps;
use super::style;
use crate::components::app::components::shell::components::toasts::ToastType;
use tw_macro::ClassList;

/// The icon's presentation: the type-tinted circle class and the glyph markup for
/// that type. Both are chosen from the toast type here, never in the body.
pub struct ToastIconPresentation {
    pub class: ClassList,
    pub svg: &'static str,
}

impl From<&ToastIconProps> for ToastIconPresentation {
    fn from(props: &ToastIconProps) -> Self {
        let toast_type = props.toast_type;
        let class = style::class(toast_type);
        let svg = match toast_type {
            ToastType::Success => TOAST_ICON_SUCCESS,
            ToastType::Error => TOAST_ICON_ERROR,
            ToastType::Warning => TOAST_ICON_WARNING,
            ToastType::Info => TOAST_ICON_INFO,
        };
        Self { class, svg }
    }
}
