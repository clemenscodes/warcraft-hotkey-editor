use tw_macro::tw;
// A horizontal strip holding the mode toggle and race tabs, with a clamped height so
// the banners keep a consistent size. On phones it stacks into a column and drops the
// min-height.

classes! {
    base: tw![
        "flex",
        "items-stretch",
        "flex-none",
        "gap-6",
        "min-h-36",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:min-h-0",
        "mobile:gap-3.5",
    ],
}
