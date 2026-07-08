use dioxus::prelude::*;

/// What a dialog composes the shell with: the open signal it drives, the title
/// the header shows, and the body to render. The shell owns everything else. A
/// dialog is just a component that fills these in; it injects no class.
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    pub open: Signal<bool>,
    #[props(into)]
    pub title: String,
    pub children: Element,
    /// An optional override for the open-change handler. `None` writes the open
    /// signal directly. A dialog with a nested child dialog passes one that
    /// guards the close, so dismissing the child does not also dismiss this one.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,
}

/// The backdrop's derived inputs: the current open value and the change handler
/// that writes the signal back. Built by `From` so the body only places them.
pub(super) struct DialogChrome {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

impl From<&DialogProps> for DialogChrome {
    fn from(props: &DialogProps) -> Self {
        let mut open_signal = props.open;
        let open = open_signal();
        let on_open_change = props
            .on_open_change
            .unwrap_or_else(|| Callback::new(move |is_open| open_signal.set(is_open)));
        Self {
            open,
            on_open_change,
        }
    }
}
