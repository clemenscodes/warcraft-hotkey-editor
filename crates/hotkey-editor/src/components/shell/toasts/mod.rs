use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;

#[derive(Props, Clone)]
pub struct ToastMountProps {
    pub children: Element,
}

impl PartialEq for ToastMountProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Wraps the app in the toast provider. The toast styling targets the primitive's
/// own `.dx-toast-*` markup and is a global concern, so it lives in the design layer
/// (`tailwind.input.css`) rather than here.
#[component]
pub fn ToastMount(props: ToastMountProps) -> Element {
    let children = props.children;
    rsx! {
        ToastProvider { {children} }
    }
}
