use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeadingProps, GoldHeadingVariant,
};
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::toast_content::ToastContentProps;
use crate::components::app::components::shell::components::toasts::ToastType;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastTitleProps {
    pub title: String,
    pub toast_type: ToastType,
}

impl From<&ToastContentProps> for ToastTitleProps {
    fn from(props: &ToastContentProps) -> Self {
        let title = props.title.clone();
        let toast_type = props.toast_type;
        Self { title, toast_type }
    }
}

impl From<&ToastTitleProps> for GoldHeadingProps {
    fn from(props: &ToastTitleProps) -> Self {
        let title = props.title.clone();
        let variant = GoldHeadingVariant::Toast;
        Self { title, variant }
    }
}
