use tw_macro::tw;

classes! {
    base: tw![
        "flex-1",
        "min-h-0",
        "overflow-y-auto",
        "flex",
        "flex-col",
        "gap-1.5",
        "px-4",
        "py-3.5",
        "bg-warcraft-bg-base/35",
        "border-l-2",
        "border-(--race-color,var(--color-warcraft-gold))",
        "rounded-control",
        "text-warcraft-text-secondary",
        "text-xl",
        "leading-prose",
        "[&>p]:whitespace-pre-wrap",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:gap-[0.3em]",
        "mobile:px-[0.7em]",
        "mobile:py-[0.5em]",
        "mobile:text-[1.1em]",
        "mobile:leading-heading",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:overflow-visible",
        "tablet:max-h-none",
        "tablet:text-xs",
        "tablet:leading-heading",
    ],
}
