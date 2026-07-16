use tw_macro::tw;

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
        "border-(--race-color,var(--color-race-human))",
        "rounded-l-control",
        "text-warcraft-text-secondary",
        "text-lg",
        "leading-prose",
    ],
    mobile: tw![
        "mobile:gap-[0.4em]",
        "mobile:py-[0.5em]",
        "mobile:pl-[0.7em]",
        "mobile:pr-0",
        "mobile:text-[1em]",
    ],
}
