mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_inline_icon::HelpInlineIcon;
use crate::components::dialogs::help_dialog::data::HelpSegment;
use style::CLASS;

pub use props::HelpWorkflowStepProps;

assert_component!(HelpWorkflowStep);

/// One step in the workflow list. A pure loop over its segments: a text run is
/// printed, an icon segment renders an inline glyph.
#[component]
pub fn HelpWorkflowStep(props: HelpWorkflowStepProps) -> Element {
    rsx! {
        li {
            class: CLASS,
            for (index, segment) in props.segments.iter().copied().enumerate() {
                {
                    match segment {
                        HelpSegment::Text { content } => rsx! { "{content}" },
                        HelpSegment::Icon { svg } => rsx! {
                            HelpInlineIcon { key: "{index}", icon: svg }
                        },
                    }
                }
            }
        }
    }
}
