use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "z-2",
        "py-1.5",
        "px-2.5",
        "pb-2",
        "w-full",
        "min-w-0",
        "text-[var(--label-color,var(--color-white))]",
    ],
    mobile: tw![
        "mobile:pt-1",
        "mobile:px-0.5",
        "mobile:pb-2",
        "mobile:text-xs",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:pt-1",
        "tablet:px-0.5",
        "tablet:pb-2",
        "tablet:text-xs",
        "tablet:tracking-snug",
    ],
}
