use dioxus::prelude::*;

/// What a dialog variant hands the shell: the open signal it drives, the title
/// the header shows, the variant's own panel class for size overrides, and the
/// body to render. The shell owns everything else.
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    pub open: Signal<bool>,
    #[props(into)]
    pub title: String,
    /// The variant's own class, appended to `dialog-panel`. Empty when the
    /// variant needs no size override beyond the shell default.
    #[props(into, default)]
    pub panel_class: String,
    pub children: Element,
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
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        Self {
            open,
            on_open_change,
        }
    }
}
