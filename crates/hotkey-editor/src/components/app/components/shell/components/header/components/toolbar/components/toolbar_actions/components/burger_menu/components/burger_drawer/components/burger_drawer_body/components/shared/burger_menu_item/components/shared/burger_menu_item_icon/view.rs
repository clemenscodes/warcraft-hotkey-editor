/// The published `View` contract mirroring [`BurgerMenuItemIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerMenuItemIconView {
    pub svg: &'static str,
}

impl ddd::View for BurgerMenuItemIconView {}
