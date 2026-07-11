/// The published `View` contract mirroring [`BurgerMenuItemLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerMenuItemLabelView {
    pub text: String,
}

impl ddd::View for BurgerMenuItemLabelView {}
