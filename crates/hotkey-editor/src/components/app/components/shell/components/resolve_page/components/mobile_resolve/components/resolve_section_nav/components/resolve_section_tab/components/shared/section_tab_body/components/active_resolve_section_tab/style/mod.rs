use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "flex-1",
        "min-w-0",
        "flex-row",
        "items-center",
        "justify-center",
        "gap-1",
        "px-1",
        "border",
        "border-[color-mix(in_oklab,var(--reason-color)_60%,transparent)]",
        "rounded-card",
        "bg-[color-mix(in_oklab,var(--reason-color)_14%,transparent)]",
        "uppercase",
        "text-(--reason-color)",
        "text-shadow-drop",
        "transition-colors",
        "duration-fast",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
}
