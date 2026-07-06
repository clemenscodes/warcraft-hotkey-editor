use tw_macro::tw;
classes! {
    base: tw![
        "w-[clamp(5.25rem,4.3vw,7rem)]",
        "h-[clamp(5.25rem,4.3vw,7rem)]",
        "[image-rendering:auto]",
        "border-2",
        "border-warcraft-blue",
        "rounded-[4px]",
        "[box-shadow:0_0_6px_color-mix(in_oklab,var(--color-warcraft-shadow)_50%,transparent)]",
        "object-cover",
        "bg-warcraft-bg-panel/70",
        "text-transparent",
        "text-[0]",
        "leading-[0]",
    ],
    mobile: tw![
        "mobile:shrink-0",
        "mobile:self-start",
        "mobile:w-[clamp(80px,22vw,120px)]",
        "mobile:h-[clamp(80px,22vw,120px)]",
    ],
    tablet: tw![
        "tablet:shrink-0",
        "tablet:self-start",
        "tablet:w-[clamp(80px,22vw,120px)]",
        "tablet:h-[clamp(80px,22vw,120px)]",
    ],
}
