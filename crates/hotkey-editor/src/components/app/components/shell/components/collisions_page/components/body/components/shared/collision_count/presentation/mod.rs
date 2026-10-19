use super::data;
use super::model::CollisionCountModel;

pub(super) struct CollisionCountPresentation {
    pub(super) count: usize,
    pub(super) noun: &'static str,
}

impl From<&CollisionCountModel> for CollisionCountPresentation {
    fn from(props: &CollisionCountModel) -> Self {
        let count = props.count;
        let noun = match count {
            1 => data::SINGULAR,
            _ => data::PLURAL,
        };
        Self { count, noun }
    }
}

impl ddd::Layered for CollisionCountPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for CollisionCountPresentation {
    type Model = CollisionCountModel;
}
