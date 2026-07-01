use crate::story::Story;

/// One overarching component within a group, holding its stories in
/// registration order. A component with more than one story is collapsible;
/// a single-story component renders as a flat leaf.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoryComponent {
    name: &'static str,
    stories: Vec<Story>,
}

impl StoryComponent {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn stories(&self) -> &[Story] {
        &self.stories
    }

    pub fn is_collapsible(&self) -> bool {
        self.stories.len() > 1
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoryGroup {
    name: &'static str,
    components: Vec<StoryComponent>,
}

impl StoryGroup {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn components(&self) -> &[StoryComponent] {
        &self.components
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct StoryRegistry {
    stories: Vec<Story>,
}

impl StoryRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(mut self, story: Story) -> Self {
        self.stories.push(story);
        self
    }

    pub fn find(&self, id: &str) -> Option<Story> {
        self.stories.iter().copied().find(|story| story.id() == id)
    }

    pub fn first_id(&self) -> Option<String> {
        self.stories.first().map(Story::id)
    }

    pub fn groups(&self) -> Vec<StoryGroup> {
        let mut groups: Vec<StoryGroup> = Vec::new();
        for story in &self.stories {
            let group_name = story.group();
            let group_index = match groups.iter().position(|group| group.name == group_name) {
                Some(index) => index,
                None => {
                    let new_group = StoryGroup {
                        name: group_name,
                        components: Vec::new(),
                    };
                    groups.push(new_group);
                    groups.len() - 1
                }
            };
            let group = &mut groups[group_index];
            let component_name = story.component();
            let existing = group
                .components
                .iter_mut()
                .find(|component| component.name == component_name);
            match existing {
                Some(component) => component.stories.push(*story),
                None => {
                    let new_component = StoryComponent {
                        name: component_name,
                        stories: vec![*story],
                    };
                    group.components.push(new_component);
                }
            }
        }
        groups
    }
}

impl FromIterator<Story> for StoryRegistry {
    fn from_iter<IntoStories: IntoIterator<Item = Story>>(stories: IntoStories) -> Self {
        let collected: Vec<Story> = stories.into_iter().collect();
        Self { stories: collected }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    fn first() -> Element {
        rsx! {
            div {}
        }
    }

    fn second() -> Element {
        rsx! {
            span {}
        }
    }

    fn registry() -> StoryRegistry {
        StoryRegistry::new()
            .register(Story::new("Buttons", "Export buttons", "loaded", first))
            .register(Story::new("Buttons", "Export buttons", "no file", second))
            .register(Story::single("Buttons", "UndoRedoButtons", first))
            .register(Story::single("Cards", "Default card", first))
    }

    #[test]
    fn groups_nest_components_in_registration_order() {
        let groups = registry().groups();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name(), "Buttons");
        assert_eq!(groups[0].components().len(), 2);
        assert_eq!(groups[0].components()[0].name(), "Export buttons");
        assert_eq!(groups[0].components()[0].stories().len(), 2);
        assert_eq!(groups[1].name(), "Cards");
    }

    #[test]
    fn collapsible_only_when_more_than_one_story() {
        let groups = registry().groups();
        let export_buttons = &groups[0].components()[0];
        let undo_redo = &groups[0].components()[1];
        assert!(export_buttons.is_collapsible());
        assert!(!undo_redo.is_collapsible());
    }

    #[test]
    fn find_resolves_known_id() {
        let found = registry().find("Buttons/Export buttons/no file");
        assert_eq!(found.map(|story| story.label()), Some("no file"));
    }

    #[test]
    fn find_returns_none_for_unknown_id() {
        assert_eq!(registry().find("Nope/Missing"), None);
    }

    #[test]
    fn first_id_is_first_registered() {
        assert_eq!(
            registry().first_id(),
            Some("Buttons/Export buttons/loaded".to_string()),
        );
    }
}
