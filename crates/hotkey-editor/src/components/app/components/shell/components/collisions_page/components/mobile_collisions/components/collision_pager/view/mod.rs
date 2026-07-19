use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;

#[derive(Clone, PartialEq)]
pub struct CollisionPagerView {
    pub content: ContentModel,
}

impl ddd::View for CollisionPagerView {}
