use tw_macro::tw;

classes! {
    base: tw![
        "flex-none",
        "inline-flex",
        "items-center",
        "px-[1.35cqi]",
        "py-[0.45cqi]",
        "rounded-tile",
        "text-lg",
        "uppercase",
        "tracking-label",
        "border",
        "border-solid",
        "text-shadow-drop",
        "whitespace-nowrap",
        "text-(--reason-color)",
        "border-[color-mix(in_oklab,var(--reason-color)_60%,transparent)]",
        "bg-[color-mix(in_oklab,var(--reason-color)_12%,transparent)]",
    ],
    // The reason badge is a small label on the phone card, not a heading.
    mobile: tw![
        "mobile:text-xs",
    ],
}
