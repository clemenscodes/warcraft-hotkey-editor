mod props;
mod style;

use dioxus::prelude::*;
pub use props::SystemHotkeysBreadcrumbsTriggerCaretProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysBreadcrumbsTriggerCaret);

/// The trigger's caret glyph, flipped when the dropdown is open.
#[component]
pub fn SystemHotkeysBreadcrumbsTriggerCaret(
    props: SystemHotkeysBreadcrumbsTriggerCaretProps,
) -> Element {
    let open = props.open;
    rsx! {
        span { class: CLASS, "data-open": open, aria_hidden: "true", "\u{25BE}" }
    }
}
