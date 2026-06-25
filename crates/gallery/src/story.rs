use dioxus::prelude::*;

/// One previewable scenario: a named component rendered in isolation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[allow(unpredictable_function_pointer_comparisons)]
pub struct Story {
    group: &'static str,
    name: &'static str,
    render: fn() -> Element,
}

impl Story {
    pub const fn new(group: &'static str, name: &'static str, render: fn() -> Element) -> Self {
        Self {
            group,
            name,
            render,
        }
    }

    pub fn group(&self) -> &'static str {
        self.group
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn render(&self) -> fn() -> Element {
        self.render
    }

    pub fn id(&self) -> String {
        format!("{}/{}", self.group, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Element {
        rsx! { div {} }
    }

    #[test]
    fn id_joins_group_and_name() {
        let story = Story::new("Buttons", "Primary", sample);
        assert_eq!(story.id(), "Buttons/Primary");
    }
}
