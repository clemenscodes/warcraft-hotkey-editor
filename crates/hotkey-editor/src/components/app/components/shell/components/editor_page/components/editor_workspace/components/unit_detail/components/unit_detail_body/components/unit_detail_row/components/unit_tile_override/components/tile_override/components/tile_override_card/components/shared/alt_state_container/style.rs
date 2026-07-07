use tw_macro::tw;
// The blue-edged block that describes an ability's off-state or upgraded form. Shared
// by the alt-state and upgrade sections.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-1",
        "py-3",
        "pr-0",
        "pl-4",
        "bg-warcraft-bg-base/55",
        "border-l-2",
        "border-race-human",
        "rounded-l-control",
        "text-warcraft-text-secondary",
        "text-lg",
        "leading-prose",
    ],
}
