use tw_macro::tw;
classes! {
    base: tw![
        "m-0",
        "uppercase",
        "tracking-[0.08em]",
        "text-[2.5rem]/[1]",
        "text-warcraft-gold",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_18px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    ],
    mobile: tw![
        "mobile:text-[clamp(12px,3.2vw,18px)]/[1]",
        "mobile:tracking-[0.02em]",
        "mobile:whitespace-nowrap",
        "mobile:overflow-hidden",
        "mobile:text-ellipsis",
        "mobile:min-w-0",
        "mobile:max-w-full",
    ],
    tablet: tw![
        "tablet:text-[clamp(12px,3.2vw,18px)]/[1]",
        "tablet:tracking-[0.02em]",
        "tablet:whitespace-nowrap",
        "tablet:overflow-hidden",
        "tablet:text-ellipsis",
        "tablet:min-w-0",
        "tablet:max-w-full",
    ],
}
