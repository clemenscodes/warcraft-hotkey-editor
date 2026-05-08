use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;

const TOAST_STYLES: Asset = asset!("/src/components/toasts/toasts.css");

#[component]
pub(crate) fn ToastMount(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: TOAST_STYLES }
        ToastProvider { {children} }
    }
}
