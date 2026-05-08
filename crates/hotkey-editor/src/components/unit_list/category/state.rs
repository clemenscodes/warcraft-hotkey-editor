pub(super) struct UnitCategoryHeadingClass;

impl UnitCategoryHeadingClass {
    pub(super) fn compute(is_collapsed: bool) -> &'static str {
        if is_collapsed {
            "unit-category-heading collapsed"
        } else {
            "unit-category-heading"
        }
    }
}
