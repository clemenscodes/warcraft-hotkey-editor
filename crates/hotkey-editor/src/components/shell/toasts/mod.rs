use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;

const TOAST_STYLES: Asset = asset!("/src/components/shell/toasts/toasts.css");

#[derive(Props, Clone)]
pub(crate) struct ToastMountProps {
    pub children: Element,
}

impl PartialEq for ToastMountProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[component]
pub(crate) fn ToastMount(props: ToastMountProps) -> Element {
    let children = props.children;
    rsx! {
        document::Stylesheet { href: TOAST_STYLES }
        ToastProvider { {children} }
    }
}
