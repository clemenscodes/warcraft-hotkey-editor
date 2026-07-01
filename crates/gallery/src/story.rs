use dioxus::prelude::*;

/// One previewable scenario: a component rendered in isolation. A story belongs
/// to a `group` (the sidebar section) and a `component` (the overarching entry).
/// A component with several stories names each with a `variant`; a component with
/// a single story leaves it `None` and shows as a flat leaf.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[allow(unpredictable_function_pointer_comparisons)]
pub struct Story {
    group: &'static str,
    component: &'static str,
    variant: Option<&'static str>,
    render: fn() -> Element,
}

impl Story {
    pub const fn new(
        group: &'static str,
        component: &'static str,
        variant: &'static str,
        render: fn() -> Element,
    ) -> Self {
        Self {
            group,
            component,
            variant: Some(variant),
            render,
        }
    }

    pub const fn single(
        group: &'static str,
        component: &'static str,
        render: fn() -> Element,
    ) -> Self {
        Self {
            group,
            component,
            variant: None,
            render,
        }
    }

    pub fn group(&self) -> &'static str {
        self.group
    }

    pub fn component(&self) -> &'static str {
        self.component
    }

    pub fn variant(&self) -> Option<&'static str> {
        self.variant
    }

    pub fn render(&self) -> fn() -> Element {
        self.render
    }

    /// The sidebar row label: the variant for a state row, otherwise the
    /// component name for a single-story leaf.
    pub fn label(&self) -> &'static str {
        self.variant.unwrap_or(self.component)
    }

    pub fn id(&self) -> String {
        match self.variant {
            Some(variant) => format!("{}/{}/{}", self.group, self.component, variant),
            None => format!("{}/{}", self.group, self.component),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Element {
        rsx! {
            div {}
        }
    }

    #[test]
    fn id_of_variant_joins_group_component_and_variant() {
        let story = Story::new("Actions", "Export buttons", "loaded", sample);
        assert_eq!(story.id(), "Actions/Export buttons/loaded");
    }

    #[test]
    fn id_of_single_joins_group_and_component() {
        let story = Story::single("Shell", "Footer", sample);
        assert_eq!(story.id(), "Shell/Footer");
    }

    #[test]
    fn label_prefers_variant_then_component() {
        let variant = Story::new("Actions", "Export buttons", "loaded", sample);
        let single = Story::single("Shell", "Footer", sample);
        assert_eq!(variant.label(), "loaded");
        assert_eq!(single.label(), "Footer");
    }
}
