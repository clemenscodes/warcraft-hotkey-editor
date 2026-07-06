mod props;
mod style;
use dioxus::prelude::*;
pub use props::CollisionsShellProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionsShell);

/// The collisions page shell: a breadcrumb bar above the two-pane content, filling
/// the view height so the content pane keeps its own scroll.
#[component]
pub fn CollisionsShell(props: CollisionsShellProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
