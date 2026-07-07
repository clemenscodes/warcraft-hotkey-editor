use tw_macro::tw;
// The blue-edged block that describes an ability's off-state or upgraded form. Shared
// by the alt-state and upgrade sections.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-[0.3rem]",
        "py-[0.7rem]",
        "pr-0",
        "pl-4",
        "bg-warcraft-bg-base/55",
        "border-l-2",
        "border-race-human",
        "rounded-[4px_0_0_4px]",
        "text-warcraft-text-secondary",
        "text-lg",
        "leading-prose",
    ],
}
