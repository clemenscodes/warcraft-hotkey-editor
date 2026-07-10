use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

/// The domain view one template card is built from: its name, description, resolved
/// layout, and apply handler. The templates dialog's hook resolves one of these per
/// bundled template and threads them down to the gallery, which renders a `TemplateCard`
/// from each by named fields.
#[derive(Clone, PartialEq)]
pub struct TemplateCardView {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}
