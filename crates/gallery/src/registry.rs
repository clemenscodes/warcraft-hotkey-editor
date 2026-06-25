use crate::story::Story;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoryGroup {
    name: &'static str,
    stories: Vec<Story>,
}

impl StoryGroup {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn stories(&self) -> &[Story] {
        &self.stories
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
            let existing = groups.iter_mut().find(|group| group.name == story.group());
            match existing {
                Some(group) => group.stories.push(*story),
                None => {
                    let new_group = StoryGroup {
                        name: story.group(),
                        stories: vec![*story],
                    };
                    groups.push(new_group);
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
        rsx! { div {} }
    }

    fn second() -> Element {
        rsx! { span {} }
    }

    fn registry() -> StoryRegistry {
        StoryRegistry::new()
            .register(Story::new("Buttons", "Primary", first))
            .register(Story::new("Buttons", "Disabled", second))
            .register(Story::new("Cards", "Default", first))
    }

    #[test]
    fn groups_preserve_order_and_membership() {
        let groups = registry().groups();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name(), "Buttons");
        assert_eq!(groups[0].stories().len(), 2);
        assert_eq!(groups[1].name(), "Cards");
        assert_eq!(groups[1].stories().len(), 1);
    }

    #[test]
    fn find_resolves_known_id() {
        let found = registry().find("Buttons/Disabled");
        assert_eq!(found.map(|story| story.name()), Some("Disabled"));
    }

    #[test]
    fn find_returns_none_for_unknown_id() {
        assert_eq!(registry().find("Nope/Missing"), None);
    }

    #[test]
    fn first_id_is_first_registered() {
        assert_eq!(registry().first_id(), Some("Buttons/Primary".to_string()));
    }
}
